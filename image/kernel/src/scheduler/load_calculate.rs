use alloc::vec::Vec;

use fixed::types::U53F11;
use hermit_sync::SpinMutex;

use crate::arch::kernel::core_local::core_id;
use crate::scheduler::{CoreId, PerCoreScheduler};

const EXP_1: f64 = 0.920_044_415;
const EXP_5: f64 = 0.983_471_454;
const EXP_15: f64 = 0.994_459_811;

#[derive(Default, Clone, Copy)]
pub(crate) struct CpuLoad {
	pub avg_1: U53F11,
	pub avg_5: U53F11,
	pub avg_15: U53F11,
}

#[derive(Default, Clone)]
struct LoadAvg {
	avg_1_fixed: U53F11,
	avg_5_fixed: U53F11,
	avg_15_fixed: U53F11,
}

static PER_CPU_LOAD: SpinMutex<Vec<LoadAvg>> = SpinMutex::new(Vec::new());

fn calc_load(avg_fixed: U53F11, active_tasks: U53F11, exp: U53F11) -> U53F11 {

    let fixed_1: u64 = 1 << 11;

    let avg_fixed_raw = avg_fixed.to_bits();
    let active_tasks_raw = active_tasks.to_bits();
    let exp_raw = exp.to_bits();

    // let mut new_load: U53F11 = avg_fixed * exp + (FixedU64::from_num(1.0) - exp) * FixedU32::from_num(active_tasks);
    let mut new_load_raw = avg_fixed_raw * exp_raw + active_tasks_raw * (fixed_1 - exp_raw);

    if active_tasks_raw >= avg_fixed_raw {
        new_load_raw += fixed_1 - 1;
        }
    
    U53F11::from_bits(new_load_raw / fixed_1)
}

impl LoadAvg {
	fn update(&mut self, active_tasks: u32) {
		let active: U53F11 = U53F11::from_num(active_tasks);
        // Maybe constants get converrted at compile time? 

		let exp_1: U53F11 = U53F11::from_num(EXP_1);
		let exp_5: U53F11 = U53F11::from_num(EXP_5);
		let exp_15: U53F11 = U53F11::from_num(EXP_15);

        self.avg_1_fixed = calc_load(self.avg_1_fixed, active, exp_1);
        self.avg_5_fixed = calc_load(self.avg_5_fixed, active, exp_5);
        self.avg_15_fixed = calc_load(self.avg_15_fixed, active, exp_15);
    }


	fn snapshot(&self) -> CpuLoad {
		CpuLoad {
			avg_1: self.avg_1_fixed,
			avg_5: self.avg_5_fixed,
			avg_15: self.avg_15_fixed,
		}
	}
}


fn check_core(core: CoreId) {
	let mut loads = PER_CPU_LOAD.lock();
	let index = usize::try_from(core).unwrap();
	if loads.len() <= index {
		loads.resize(index + 1, LoadAvg::default());
	}
}

/// Update the load average for the current core from its scheduler queues.
pub fn update_for_core(scheduler: &PerCoreScheduler) {
	let core = core_id();
	check_core(core);

	let active_tasks = scheduler.count_active_tasks();
	let snapshot = {
		let mut loads = PER_CPU_LOAD.lock();
		let entry = &mut loads[usize::try_from(core).unwrap()];
		entry.update(active_tasks);
		entry.snapshot()
	};

	info!(
		"load on core {core}: active={active_tasks} avg_1={:.2} avg_5={:.2} avg_15={:.2}",
		snapshot.avg_1.to_num::<f32>(),
		snapshot.avg_5.to_num::<f32>(),
		snapshot.avg_15.to_num::<f32>(),
	);
}

/// Returns the latest load averages for `core`, if that core has been sampled.
pub fn get_cpu_load(core: CoreId) -> Option<CpuLoad> {
	let loads = PER_CPU_LOAD.lock();
	let index = usize::try_from(core).ok()?;
	loads.get(index).map(LoadAvg::snapshot)
}

pub fn collect_cpu_loads(num_cores: u32) -> Vec<(CoreId, CpuLoad)> {
    let mut loads = Vec::new();
    for core in 0..num_cores {
        if let Some(load) = get_cpu_load(core) {
            loads.push((core, load));
        }
    }
    loads
}

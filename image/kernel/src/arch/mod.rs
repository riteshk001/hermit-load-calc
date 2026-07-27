//! Architecture-specific architecture abstraction.

cfg_select! {
	target_arch = "aarch64" => {
		mod aarch64;
		pub(crate) use self::aarch64::*;
		pub use self::aarch64::mm::paging::{BasePageSize, PageSize};
		pub use self::aarch64::mm::clear_user_space;
	}
	target_arch = "riscv64" => {
		mod riscv64;
		pub(crate) use self::riscv64::*;
	}
	target_arch = "x86_64" => {
		mod x86_64;
		pub(crate) use self::x86_64::*;
		pub use self::x86_64::mm::{BasePageSize, PageSize, clear_user_space};
	}
}

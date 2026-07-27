//
// EVERYTHING BELOW THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The Continuous Integration platform detected during compilation."#]
#[allow(dead_code)]
pub static CI_PLATFORM: Option<&str> = None;
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The full version."#]
#[allow(dead_code)]
pub static PKG_VERSION: &str = "0.13.0";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The major version."#]
#[allow(dead_code)]
pub static PKG_VERSION_MAJOR: &str = "0";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The minor version."#]
#[allow(dead_code)]
pub static PKG_VERSION_MINOR: &str = "13";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The patch version."#]
#[allow(dead_code)]
pub static PKG_VERSION_PATCH: &str = "0";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The pre-release version."#]
#[allow(dead_code)]
pub static PKG_VERSION_PRE: &str = "";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"A colon-separated list of authors."#]
#[allow(dead_code)]
pub static PKG_AUTHORS: &str = "The Hermit Project Developers";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The name of the package."#]
#[allow(dead_code)]
pub static PKG_NAME: &str = "hermit-kernel";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The description."#]
#[allow(dead_code)]
pub static PKG_DESCRIPTION: &str = "A Rust-based library operating system";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The homepage."#]
#[allow(dead_code)]
pub static PKG_HOMEPAGE: &str = "";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The license."#]
#[allow(dead_code)]
pub static PKG_LICENSE: &str = "MIT OR Apache-2.0";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The source repository as advertised in Cargo.toml."#]
#[allow(dead_code)]
pub static PKG_REPOSITORY: &str = "https://github.com/hermit-os/kernel";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The target triple that was being compiled for."#]
#[allow(dead_code)]
pub static TARGET: &str = "x86_64-unknown-none";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The host triple of the rust compiler."#]
#[allow(dead_code)]
pub static HOST: &str = "x86_64-unknown-linux-gnu";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"`release` for release builds, `debug` for other builds."#]
#[allow(dead_code)]
pub static PROFILE: &str = "debug";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The compiler that cargo resolved to use."#]
#[allow(dead_code)]
pub static RUSTC: &str = "rustc";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The documentation generator that cargo resolved to use."#]
#[allow(dead_code)]
pub static RUSTDOC: &str = "rustdoc";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"Value of `OPT_LEVEL` for the profile used during compilation."#]
#[allow(dead_code)]
pub static OPT_LEVEL: &str = "0";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The parallelism that was specified during compilation."#]
#[allow(dead_code)]
pub static NUM_JOBS: u32 = 16;
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"Value of DEBUG for the profile used during compilation."#]
#[allow(dead_code)]
pub static DEBUG: bool = true;
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The features that were enabled during compilation."#]
#[allow(dead_code)]
pub static FEATURES: [&str; 17] = ["acpi", "common-os", "dhcpv4", "dns", "fork", "fsgsbase", "kernel-stack", "net", "pci", "pci-ids", "semihosting", "smoltcp", "tcp", "udp", "virtio", "virtio-fs", "virtio-net"];
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The features as a comma-separated string."#]
#[allow(dead_code)]
pub static FEATURES_STR: &str = "acpi, common-os, dhcpv4, dns, fork, fsgsbase, kernel-stack, net, pci, pci-ids, semihosting, smoltcp, tcp, udp, virtio, virtio-fs, virtio-net";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The features as above, as lowercase strings."#]
#[allow(dead_code)]
pub static FEATURES_LOWERCASE: [&str; 17] = ["acpi", "common-os", "dhcpv4", "dns", "fork", "fsgsbase", "kernel-stack", "net", "pci", "pci-ids", "semihosting", "smoltcp", "tcp", "udp", "virtio", "virtio-fs", "virtio-net"];
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The feature-string as above, from lowercase strings."#]
#[allow(dead_code)]
pub static FEATURES_LOWERCASE_STR: &str = "acpi, common-os, dhcpv4, dns, fork, fsgsbase, kernel-stack, net, pci, pci-ids, semihosting, smoltcp, tcp, udp, virtio, virtio-fs, virtio-net";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The output of `rustc -V`"#]
#[allow(dead_code)]
pub static RUSTC_VERSION: &str = "rustc 1.96.0-nightly (48cc71ee8 2026-03-31)";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The output of `rustdoc -V`; empty string if `rustdoc -V` failed to execute"#]
#[allow(dead_code)]
pub static RUSTDOC_VERSION: &str = "rustdoc 1.96.0-nightly (48cc71ee8 2026-03-31)";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The target architecture, given by `CARGO_CFG_TARGET_ARCH`."#]
#[allow(dead_code)]
pub static CFG_TARGET_ARCH: &str = "x86_64";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The endianness, given by `CARGO_CFG_TARGET_ENDIAN`."#]
#[allow(dead_code)]
pub static CFG_ENDIAN: &str = "little";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`."#]
#[allow(dead_code)]
pub static CFG_ENV: &str = "";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The OS-family, given by `CARGO_CFG_TARGET_FAMILY`."#]
#[allow(dead_code)]
pub static CFG_FAMILY: &str = "";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The operating system, given by `CARGO_CFG_TARGET_OS`."#]
#[allow(dead_code)]
pub static CFG_OS: &str = "none";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`."#]
#[allow(dead_code)]
pub static CFG_POINTER_WIDTH: &str = "64";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_VERSION` contains HEAD's tag. The short commit id is used if HEAD is not tagged."#]
#[allow(dead_code)]
pub static GIT_VERSION: Option<&str> = Some("fc8c300");
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"If the repository had dirty/staged files."#]
#[allow(dead_code)]
pub static GIT_DIRTY: Option<bool> = Some(true);
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_HEAD_REF` contains full name to the reference pointed to by HEAD (e.g.: `refs/heads/master`). If HEAD is detached or the branch name is not valid UTF-8 `None` will be stored.
"#]
#[allow(dead_code)]
pub static GIT_HEAD_REF: Option<&str> = None;
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH` contains HEAD's full commit SHA-1 hash."#]
#[allow(dead_code)]
pub static GIT_COMMIT_HASH: Option<&str> = Some("fc8c300b1c520a107923c7a9c11fd8c9c1ec7c56");
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"If the crate was compiled from within a git-repository, `GIT_COMMIT_HASH_SHORT` contains HEAD's short commit SHA-1 hash."#]
#[allow(dead_code)]
pub static GIT_COMMIT_HASH_SHORT: Option<&str> = Some("fc8c300");
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The build time in RFC2822, UTC."#]
#[allow(dead_code)]
pub static BUILT_TIME_UTC: &str = "Mon, 20 Jul 2026 22:18:55 +0000";
#[allow(clippy::needless_raw_string_hashes)]
#[doc=r#"The override-variables that were used during compilation."#]
#[allow(dead_code)]
pub static OVERRIDE_VARIABLES_USED: [&str; 0] = [];
//
// EVERYTHING ABOVE THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//

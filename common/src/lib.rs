use pkg_version::{pkg_version_major, pkg_version_minor, pkg_version_patch};

pub mod input;
pub mod loaders;
pub mod materials;
pub mod network;
pub mod units;

extern crate bevy;
extern crate kdl;
extern crate rand;
extern crate rand_distr;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub const fn version_num() -> u32 {
    let major: u32 = pkg_version_major!();
    let minor: u32 = pkg_version_minor!();
    let patch: u32 = pkg_version_patch!();

    major << 24 | minor << 16 | patch << 8
}

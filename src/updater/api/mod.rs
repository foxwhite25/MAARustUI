mod repo_release;

pub use repo_release::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Version {
    Stable,
    Nightly,
}

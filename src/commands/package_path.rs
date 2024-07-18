use crate::utils::{
    paths::d_packages,
    state::Result,
};

pub fn run() -> Result<bool> {
    println!("Packages are located at: '{}'", d_packages());
    Ok(true)
}

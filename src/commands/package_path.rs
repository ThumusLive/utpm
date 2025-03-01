use tracing::instrument;

use crate::utils::{paths::d_packages, state::Result};

#[instrument]
pub fn run() -> Result<bool> {
    println!("Packages are located at: '{}'", d_packages()?);
    eprintln!("{}", d_packages()?);
    Ok(true)
}

use tracing::{error, info, instrument};

use crate::utils::state::{Error, Result};

use super::{unlink, BulkDeleteArgs, UnlinkArgs};

#[instrument]
pub fn run(cmd: &BulkDeleteArgs) -> Result<bool> {
    let mut vec: Vec<Error> = Vec::new();
    for name in &cmd.names {
        match unlink::run(&UnlinkArgs {
            package: name.into(),
            yes: true,
        }) {
            Ok(_) => {
                info!("- {name} deleted");
            }
            Err(err) => {
                info!("X {name} not found");
                error!("{err}");
                vec.push(err);
            }
        };
    }
    println!(
        "{}/{} successful",
        cmd.names.len() - vec.len(),
        cmd.names.len()
    );
    Ok(true)
}

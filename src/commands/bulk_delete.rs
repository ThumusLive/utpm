use tracing::{error, instrument};

use crate::utils::state::{Error, Result};

use super::{unlink, BulkDeleteArgs, UnlinkArgs};

#[instrument]
pub fn run(cmd: &BulkDeleteArgs) -> Result<bool> {
    //todo: regex
    let mut vec: Vec<Error> = Vec::new();
    for name in &cmd.names {
        let name_and_version = name
            .split(":")
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        let ulnk = UnlinkArgs {
            delete_namespace: false,
            name: Some(name_and_version[0].to_owned()),
            yes: true,
            namespace: cmd.namespace.to_owned(),
            version: if name_and_version.len() > 1 {
                Some(semver::Version::parse(name_and_version[1].as_str())?)
            } else {
                None
            },
        };
        match unlink::run(&ulnk) {
            Ok(_) => (),
            Err(err) => {
                error!("{}", err);
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

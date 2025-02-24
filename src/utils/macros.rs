
/// Load automatically your manifest (typst.toml).
/// Can specify (or not) the path to the manifest.
#[macro_export]
macro_rules! manifest {
    () => {
        match Manifest::try_find(get_current_dir()?)? {
            Some(val) => Ok(val),
            None => Err(Error::empty(ErrorKind::Manifest)),
        }?
    };
    ($var:expr) => {
        match Manifest::try_find($var)? {
            Some(val) => Ok(val),
            None => Err(Error::empty(ErrorKind::Manifest)),
        }?
    };
}

/// Write data to your manifest (typst.toml) 
/// Can specify (or not) your path. Data must be provided.
#[macro_export]
macro_rules! write_manifest {
    ($var:expr => $path:expr) => {
        let tomlfy: String = toml::to_string_pretty($var)?;
        fs::write($path, tomlfy)?
    };
    ($var:expr) => {
        let tomlfy: String = toml::to_string_pretty($var)?;
        fs::write("./typst.toml", tomlfy)?
    };
}

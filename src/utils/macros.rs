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

#[macro_export]
macro_rules! write_manifest {
    ($var:expr => $path:expr) => {
        let tomlfy: String = toml::to_string_pretty($var)?;
        fs::write($path, tomlfy)?
    };
    ($var:expr) => {
        let tomlfy: String = toml::to_string_pretty($var)?;
        fs::write(get_current_dir()? + "/typst.toml", tomlfy)?
    };
}

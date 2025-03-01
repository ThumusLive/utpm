/// Load automatically your manifest (typst.toml).
/// Can specify (or not) the path to the manifest.
#[macro_export]
macro_rules! load_manifest {
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


/// Get the path of a package
#[macro_export]
macro_rules! format_package {
    ($namespace:expr) => {{
        (format!(
            "{}/{}",
            if $namespace == "preview" {
                info!("preview found, cache dir use");
                c_packages()?
            } else {
                info!("no preview found, data dir use");
                d_packages()?
            },
            $namespace
        ))
    }};

    ($namespace:expr, $package:expr) => {{
        (format!("{}/{}", format_package!($namespace), $package))
    }};

    ($namespace:ident, $package:ident, $major:ident, $minor:ident, $patch:ident) => {{
        (format!(
            "{}/{}.{}.{}",
            format_package!($namespace, $package),
            $major,
            $minor,
            $patch
        ))
    }};
}

/// Load ssh credentials
#[macro_export]
macro_rules! load_creds {
    ($callbacks:expr, $val:expr) => {{
        $callbacks.credentials(|_, username_from_url, _| {
            let binding: String =
                env::var("UTPM_USERNAME").unwrap_or(username_from_url.unwrap_or("git").to_string());
            let username: &str = binding.as_str();
            match Cred::ssh_key_from_agent(username) {
                Ok(cred) => Ok(cred),
                Err(_) => Ok(match env::var("UTPM_PASSPHRASE") {
                    Ok(s) => {
                        info!(passphrase = true);
                        Cred::ssh_key(username, None, Path::new(&$val), Some(s.as_str()))?
                    }
                    Err(_) => {
                        info!(passphrase = false);
                        Cred::ssh_key(username, None, Path::new(&$val), None)?
                    }
                }),
            }
        });
    }};
}

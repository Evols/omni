use anyhow::{Result, anyhow};
use std::{env, path::PathBuf};

const APPLICATION_NAME: &str = "omni";

pub fn app_data_dir() -> Result<PathBuf> {
    Ok(platform_app_data_base()?.join(APPLICATION_NAME))
}

#[cfg(target_os = "windows")]
fn platform_app_data_base() -> Result<PathBuf> {
    if let Some(local_appdata) = env::var_os("LOCALAPPDATA") {
        return Ok(local_appdata.into());
    };

    if let Some(user_profile) = env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(user_profile).join("AppData").join("Local"));
    }

    Err(anyhow!(
        "The LOCALAPPDATA and USERPROFILE environment variables are not set"
    ))
}

#[cfg(target_os = "macos")]
fn platform_app_data_base() -> Result<PathBuf> {
    let home = match env::var_os("HOME") {
        Some(home) => home,
        None => return Err(anyhow!("Missing $HOME in environment")),
    };

    Ok(PathBuf::from(home)
        .join("Library")
        .join("Application Support"))
}

#[cfg(all(unix, not(target_os = "macos")))]
fn platform_app_data_base() -> Result<PathBuf> {
    // XDG data dir (good default for app data / db files)
    if let Some(xdg) = env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(xdg));
    }

    if let Some(home) = env::var_os("HOME") {
        return Ok(PathBuf::from(home).join(".local").join("share"));
    }

    Err(anyhow!(
        "The XDG_DATA_HOME and HOME environment variables are not set"
    ))
}

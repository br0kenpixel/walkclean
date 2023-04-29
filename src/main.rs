use log::{debug, error, info, log_enabled, warn, Level, LevelFilter};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;
#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(LOG_LEVEL)
        .init();

    info!("Starting walkclean v{}", env!("CARGO_PKG_VERSION"));

    if log_enabled!(Level::Debug) {
        warn!("This is a debug build!");
    }

    let workdir = current_dir().unwrap();
    info!("Starting cleanup from \"{}\"", workdir.display());

    for entry in WalkDir::new(&workdir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                error!("Failed to walk path: {e}");
                continue;
            }
        };
        let path_display = entry.path().display();
        let entry_path = entry.path().to_path_buf();

        if entry_path == workdir || !entry_path.is_dir() {
            continue;
        }

        if log_enabled!(Level::Debug) {
            debug!("Walked into \"{}\"", path_display);
        }

        if is_cargo_dir(&entry_path) {
            if log_enabled!(Level::Debug) {
                debug!("Found Cargo project in \"{path_display}\"");
            }

            match clean_project_dir(&entry_path) {
                Ok(success) => {
                    if success {
                        info!("Successfully cleaned \"{path_display}\"");
                    } else {
                        warn!("Clean command failed in \"{path_display}\"");
                    }
                }
                Err(e) => error!("Failed to clean \"{path_display}\": {e}"),
            };
        }
    }

    info!("Done");
}

fn is_cargo_dir(path: &Path) -> bool {
    let mut target_dir_found = false;
    let mut cargo_cfg_found = false;

    let dir_iterator = match path.read_dir() {
        Ok(iter) => iter,
        Err(e) => {
            error!("Failed to read directory at \"{}\": {e}", path.display());
            return false;
        }
    };
    for file in dir_iterator {
        let file = file.unwrap();
        let f_name = file.file_name();
        let f_type = file.file_type().unwrap();

        if f_name == "target" && f_type.is_dir() {
            target_dir_found = true;
        }

        if f_name == "Cargo.toml" && f_type.is_file() {
            cargo_cfg_found = true;
        }
    }

    target_dir_found && cargo_cfg_found
}

fn clean_project_dir(path: &PathBuf) -> Result<bool, String> {
    Command::new("cargo")
        .arg("clean")
        .current_dir(path)
        .status()
        .map_err(|e| format!("{e}"))
        .map(|status| status.success())
}

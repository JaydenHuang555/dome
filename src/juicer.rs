use std::{fs, io, process::ExitCode, sync::OnceLock};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

use crate::options::Action;
use crate::{config::Config, options::Options, paths, profile::Profile};

use crate::sync;

pub fn execute(action: &Action, profile: &Profile) {
    match action {
        Action::Download(inputs) => {}
        Action::List => {}
    }
}

pub fn run(options: &Options) -> ExitCode {
    // ensure directories are set
    if let Err(e) = fs::create_dir_all(paths::config_path().parent().unwrap()) {
        eprintln!("Unable to create config dir due to {}", e);
        return ExitCode::from(10);
    }
    if let Err(e) = fs::create_dir_all(paths::profiles_path()) {
        eprintln!("Unable to create profile directory due to {}", e);
        return ExitCode::from(5);
    }

    // aquire config
    let mut config = sync::verify_config();

    let loaded_profile;

    // profile is passed in
    if let Some(passed) = options.passed() {
        loaded_profile = Profile::from(passed.clone());
        if let Some(e) = sync::unload_profile(&loaded_profile) {
            return e;
        }
        // assign profile
        config.set_current_profile(passed.clone().name.unwrap());
        sync::unload_config(&config);
    }
    // profile is the current
    else {
        let cur = sync::current_profile(&mut config);
        if let Err(e) = cur {
            return e;
        }
        loaded_profile = cur.unwrap();
        config.set_current_profile(loaded_profile.name());
    }

    println!("{:?}", loaded_profile);

    // execute action
    execute(&options.action, &loaded_profile);

    ExitCode::SUCCESS
}

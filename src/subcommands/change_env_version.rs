use std::env;

use clap::Parser;
use crate::subcommands::path_functions::get_config_path;

use super::unreal_structs::{CollectionOfVersions};

#[derive(Parser, Debug)]
pub struct ChangeEnv {
    #[clap(long="version", short='v')]
    version: String,
}

impl ChangeEnv {
    fn change_unreal_env(&self, unreal_dir: String) {
        env::set_var("UNREAL_HOME", unreal_dir)
    }
    pub fn change_version(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        println!("{}", config_path);
        let config_result = std::fs::File::open(config_path)?;
        let config_versions: CollectionOfVersions = serde_yaml::from_reader(config_result)?;
        for unreal_version in config_versions.versions {
            if unreal_version.version == self.version {
                self.change_unreal_env(unreal_version.path);
                return Ok(());
            }
        }
        Err("The specified unreal version is not registered")?
    }
}
use super::unreal_structs::{CollectionOfVersions, UnrealVersion};
use super::path_functions::{get_config_path};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Register {
    #[clap(long = "version", short = 'v')]
    version: String,
    #[clap(long = "path", short = 'p')]
    path_to_bin: String,
}

impl Register {
    pub fn register_version(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        let config_result = std::fs::File::open(String::from(&config_path));
        let mut config_versions: CollectionOfVersions = CollectionOfVersions {
            versions: Vec::new(),
        };
        let unreal_version: UnrealVersion = UnrealVersion {
            version: String::from(&self.version),
            path: String::from(&self.path_to_bin),
        };
        let mut has_changed = false;
        if config_result.is_ok() {
            config_versions = serde_yaml::from_reader(config_result.ok().unwrap())?;

            for version in config_versions.versions.iter_mut() {
                if version.version == self.version {
                    *version = unreal_version.clone();
                    has_changed = true;
                    break;
                }
            }
        }
        if !has_changed {
            config_versions.versions.push(unreal_version);
        }
        let config_string = serde_yaml::to_string(&config_versions)?;
        std::fs::write(String::from(&config_path), config_string)?;
        Ok(())
    }
}

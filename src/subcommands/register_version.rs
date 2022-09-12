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
        let config_result = std::fs::File::open(config_path);
        let mut config_versions: CollectionOfVersions = CollectionOfVersions {
            versions: Vec::new(),
        };
        let mut has_changed = false;
        if config_result.is_ok() {
            config_versions = serde_yaml::from_reader(config_result.ok().unwrap())?;

            let _ = &config_versions.versions.iter_mut().map(|version| {
                if version.version == self.version {
                    version.path = String::from(&self.path_to_bin);
                    has_changed = true;
                }
            });
        }
        if !has_changed {
            let unreal_version: UnrealVersion = UnrealVersion {
                version: String::from(&self.version),
                path: String::from(&self.path_to_bin),
            };
            config_versions.versions.push(unreal_version);
        }
        let config_string = serde_yaml::to_string(&config_versions)?;
        std::fs::write("config.yaml", config_string)?;
        Ok(())
    }
}

use clap::Parser;
use cli_table::print_stdout;

use super::{path_functions::get_config_path, unreal_structs::CollectionOfVersions};


#[derive(Parser,Debug)]
pub struct ListOfVersions {
    
}

impl ListOfVersions {
    pub fn get_list(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        println!("{}", config_path);
        let config_result = std::fs::File::open(config_path)?;
        let config_versions: CollectionOfVersions = serde_yaml::from_reader(config_result)?;
        assert!(print_stdout(config_versions.versions).is_ok());
        Ok(())
    }
}
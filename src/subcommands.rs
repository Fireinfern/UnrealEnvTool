use clap::Subcommand;

use self::{register_version::Register, change_env_version::ChangeEnv, get_list::ListOfVersions};

mod register_version;
mod change_env_version;
mod get_list;
mod unreal_structs;
mod path_functions;

#[derive(Subcommand, Debug)]
pub enum Commands{
    Register(Register),
    ChangeEnv(ChangeEnv),
    ListOfVersions(ListOfVersions)
}
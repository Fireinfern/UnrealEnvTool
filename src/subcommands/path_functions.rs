pub fn get_config_path() -> Result<String, Box<dyn std::error::Error>>{
    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("config.yaml");
    let config_path = path.into_os_string().into_string().unwrap();
    Ok(config_path)
}
use common::config::Config;

#[test]
fn test_config() {
    let config = Config::new().unwrap();
    println!("config: {:?}", config);
    // Ensure that the database name is not the default value
    assert_ne!(config.database.name, "postgres");
}

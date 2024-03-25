use common::config::Config;

#[test]
fn test_config() {
    let config = Config::new().unwrap();
    println!("config: {:?}", config);
}

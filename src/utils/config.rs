#[macro_export]
macro_rules! load_config {
    () => {
        let enviroment = std::env::var("ENV");
        match enviroment {
            Ok(val) => {
                if val.eq("prod") {
                    dotenv::from_filename("env/.prod.env").ok();
                } else if val.eq("stag") {
                    dotenv::from_filename("env/.stag.env").ok();
                } else {
                    dotenv::from_filename("env/.dev.env").ok();
                }
            }
            Err(_) => {
                dotenv::from_filename("env/.dev.env").ok();
            }
        }
        dotenv::from_filename("env/.env").ok();
    };
}

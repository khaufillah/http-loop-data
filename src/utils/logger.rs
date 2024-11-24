#[macro_export]
macro_rules! info {
    ($msg:expr) => {
        let now = chrono::prelude::Utc::now().to_string();
        log::info!("{} | {}", now, $msg)
    };
}

#[macro_export]
macro_rules! warn {
    ($msg:expr) => {
        let now = chrono::prelude::Utc::now().to_string();
        log::warn!("{} | {}", now, $msg)
    };
}

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        let now = chrono::prelude::Utc::now().to_string();
        log::error!("{} | {}", now, $msg)
    };
}

#[macro_export]
macro_rules! panic {
    ($msg:expr) => {
        let now = chrono::prelude::Utc::now().to_string();
        log::error!("{} | {}", now, $msg);
        std::panic!()
    };
}

use chrono::Utc;
use log::LevelFilter;
use std::io::Write;

pub fn init() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {} - {}",
                Utc::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

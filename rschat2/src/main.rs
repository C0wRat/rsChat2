use env_logger::Builder;
use log::{info, warn, error};
use log::LevelFilter;
use std::io::Write;

struct RS2{}


impl RS2{
    fn setup_logger() {
        Builder::from_default_env()
        .format(|buf, record| {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(
                buf,
                "[{}] {} {} > {}",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info) // Set default log level to INFO
        .init();
    }    
}


fn main() {
    RS2::setup_logger();
    info!("RS2 Starting...")
}
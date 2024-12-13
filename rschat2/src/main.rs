use log::{info, warn, error};

use rs2_tui::start_tui;
use rs2_log;

fn main() {
    rs2_log::setup_logger();
    info!("RS2 Starting...");
    start_tui();
}
use wayland_client::Connection;
use clap::{Parser};

mod appdata;
mod config_manipulator;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // NOTE: change type to PathBuf?
    compositor_config_file_path: String,
    config_file_path: String,
}

fn main() {
    let mut app_state = appdata::AppData::default();
    parse_program_args(&mut app_state);

    infinite_wayland(&mut app_state);
}

fn parse_program_args(app_state: &mut appdata::AppData) {
    let args = Args::parse();

    app_state.compositor_config_file_path = args.compositor_config_file_path;
    
    app_state.config_file_path = args.config_file_path;
    app_state.check_for_builtin_monitor();
}

fn infinite_wayland(app_state: &mut appdata::AppData) {
    let conn = Connection::connect_to_env().unwrap();

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let display = conn.display();
    let _registry = display.get_registry(&qh, ());

    // registry roundrip for binding to events
    event_queue.roundtrip(app_state).unwrap();

    loop {
        // check for new (wl_output) events
        event_queue.blocking_dispatch(app_state).unwrap();
    }
}

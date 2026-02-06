use std::collections::HashMap;
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle,
    backend::ObjectId,
    protocol::{wl_output, wl_registry},
};

use crate::config_manipulator::{momand, niri::modify_niri_config};

#[derive(Default)]
pub struct AppData {
    pub output_event_binds: HashMap<ObjectId, u32>,
    pub output_names: HashMap<u32, String>,
    pub compositor_config_file_path: String,
    pub config_file_path: String,

    builtin_screen: Option<String>
}

impl AppData {
    pub fn check_for_builtin_monitor(&mut self) {
        self.builtin_screen = momand::config_builtin_monitor(&self.config_file_path)
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        match event {
            wl_registry::Event::Global {
                name,
                interface,
                version,
            } => {
                if interface == "wl_output" {
                    // TODO: check the different versions of wl_ouput interface in docs for
                    // backward compat
                    let output = registry.bind::<wl_output::WlOutput, _, _>(name, version, qh, ());
                    println!("new device connected. Has name: {}", name);
                    state.output_event_binds.insert(output.id(), name);
                }
            }
            wl_registry::Event::GlobalRemove { name } => {
                // if race condition occurs where this event fires before Name:
                state.output_event_binds.retain(|_, global| *global != name);

                let Some(_) = state.output_names.remove(&name) else { return; };
                println!("monitor removed from output_names list: {}", name);

                match state.output_names.len() {
                    1 => {
                        let output_name = state.output_names.values().next().unwrap();
                        modify_niri_config(&state.compositor_config_file_path, output_name, true);
                    },
                    0 => {
                        if let Some(monitor_name) = state.builtin_screen.as_deref() {
                            modify_niri_config(&state.compositor_config_file_path, monitor_name, true);
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_output::WlOutput, ()> for AppData {
    fn event(
        state: &mut Self,
        output: &wl_output::WlOutput,
        event: <wl_output::WlOutput as Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            wl_output::Event::Name { name } => {
                println!("wl_output name event for {}", name);
                if let Some(global_name) = state.output_event_binds.remove(&output.id()) {
                    state.output_names.insert(global_name, name);
                }

                // NOTE: do nothing else?
            }
            _ => {}
        }
    }
}

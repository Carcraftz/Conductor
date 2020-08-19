use std::sync::{Arc, RwLock};
use crate::state::State;
use actix::Addr;
use crate::webserver::WebsocketHandler;
use std::thread;
use std::time::Duration;

#[path = "mac/mgr.rs"]
mod mgr;

use mgr::*;
use crate::ipc;

pub fn bind_keys(state: Arc<RwLock<State>>, addr: Addr<WebsocketHandler>) -> bool {
    thread::spawn(move || {
        if let Some(mgr) = InputManager::new() {
            let mut return_pressed = false;
            let mut space_pressed = false;
            let mut enable_triggered = false;

            loop {
                if mgr.poll_enter() && !return_pressed {
                    println!("Enter pressed");
                    state.write().unwrap().disable();
                    addr.do_send(ipc::Message::UpdateEnableStatus { enabled: false, from_backend: true });
                }
                if mgr.poll_spacebar() && !space_pressed {
                    println!("Spacebar pressed");
                    let mut state = state.write().unwrap();
                    if state.ds.enabled() {
                        state.estop();
                        addr.do_send(ipc::Message::EstopRobot { from_backend: true });
                    }
                }
                if mgr.poll_enable() && !enable_triggered {
                    // state.write().unwrap().enable();
                    // addr.do_send(ipc::Message::UpdateEnableStatus { enabled: false, from_backend: true });
                    println!("Enable the robot");
                }

                return_pressed = mgr.poll_enter();
                space_pressed = mgr.poll_spacebar();
                enable_triggered = mgr.poll_enable();
                thread::sleep(Duration::from_millis(20))
            }
        } else {
            println!("Failed to crate input manager.");
            addr.do_send(ipc::Message::Capabilities { backend_keybinds: false });
        }
    });
    true
}
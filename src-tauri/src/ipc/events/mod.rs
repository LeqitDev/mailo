use std::sync::{Arc, Mutex};

use tauri::{AppHandle, Manager};

use crate::app::{events::FrontendEvent, state::Shareble};

pub fn frontend_event_dispatch_loop(app_state: Arc<Mutex<Shareble>>, handle: AppHandle) {
    log::info!("Starting frontend event dispatch loop");
    tauri::async_runtime::spawn(async move {
        loop {
            // stop the loop if the app is logging out
            if app_state.lock().unwrap().logout {
                log::info!("Stopping frontend event dispatch loop, app is logging out");
                break;
            }

            if let Ok(mut app_state) = app_state.lock() {
                // wait for the frontend to be ready
                if !app_state.frontend_ready {
                    continue;
                }

                // clone and clear the events
                let mut events = app_state.events.clone();
                app_state.events.clear();

                // remove duplicate action events
                events.dedup_by(|a, b| {
                    if let (FrontendEvent::Action(a), FrontendEvent::Action(b)) = (a, b) {
                        a.action == b.action && a.payload == b.payload
                    } else {
                        false
                    }
                });
                if !events.is_empty() {
                    log::debug!("Sending events to frontend: {:#?}", events);
                    // TODO: weird thing the third event does not fire
                    // dispatch the different events to the frontend
                    for event in events {
                        match event {
                            FrontendEvent::Log(log) => {
                                handle.get_window("main").unwrap().emit("log", log).unwrap()
                            }
                            FrontendEvent::Action(action) => handle
                                .get_window("main")
                                .unwrap()
                                .emit("action", action)
                                .unwrap(),
                            FrontendEvent::Notify(notify) => handle
                                .get_window("main")
                                .unwrap()
                                .emit("notify", notify)
                                .unwrap(),
                        }
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await; // let the thread sleep for a bit
        }
    });
}

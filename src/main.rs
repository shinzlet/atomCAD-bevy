// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate relaunch;

use relaunch::Trampoline;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

fn main() {
    const APP_NAME: &str = "atomCAD";

    // If we are running from the command line (e.g. as a result of `cargo
    // run`), relaunch as a dynamically created app bundle.  This is required
    // because many Cocoa APIs will not work unless the application is running
    // from a bundle.
    #[cfg(target_os = "macos")]
    let _ =
        match Trampoline::new(&APP_NAME, "io.atomcad.atomCAD").bundle(relaunch::InstallDir::Temp) {
            Err(e) => {
                // We can't read/write to the filesystem?  This is a fatal error.
                println!("IO error! {}", e);
                std::process::exit(1);
            }
            Ok(app) => app,
        };

    // Create the event loop.
    let mut event_loop = EventLoopBuilder::new();
    let event_loop = event_loop.build();

    // Create the main window.
    let mut window = match WindowBuilder::new().with_title(APP_NAME).build(&event_loop) {
        Err(e) => {
            println!("Failed to create window: {}", e);
            std::process::exit(1);
        }
        Ok(window) => Some(window),
    };

    // Run the event loop.
    event_loop.run(move |event, _, control_flow| {
        // When we are done handling this event, suspend until the next event.
        *control_flow = ControlFlow::Wait;

        // Handle events.
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // The user has requested to close the window.
                // Drop the window to fire the `Destroyed` event.
                window = None;
            }
            Event::WindowEvent {
                event: WindowEvent::Destroyed,
                ..
            } => {
                // The window has been destroyed, time to exit stage left.
                *control_flow = ControlFlow::ExitWithCode(0);
            }
            Event::MainEventsCleared => {
                // The event queue is empty, so we can safely redraw the window.
                if let Some(w) = &window {
                    w.request_redraw();
                }
            }
            Event::LoopDestroyed => {
                // The event loop has been destroyed, so we can safely terminate
                // the application.  This is the very last event we will ever
                // receive, so we can safely perform final rites.
            }
            _ => {
                // Unknown event; do nothing.
            }
        }
    });
}

// End of File

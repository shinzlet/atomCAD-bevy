// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(target_os = "macos")]
extern crate relaunch;
#[cfg(target_os = "macos")]
use relaunch::Trampoline;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
use objc::rc::autoreleasepool;
#[cfg(target_os = "macos")]
use objc::runtime::Object;

#[cfg(target_os = "macos")]
use winit::platform::macos::EventLoopBuilderExtMacOS;
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
    let app =
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
    #[cfg(target_os = "macos")]
    event_loop.with_default_menu(false);
    let event_loop = event_loop.build();

    // Create the menu on macOS using Cocoa APIs.
    #[cfg(target_os = "macos")]
    autoreleasepool(|| unsafe {
        // Empty string (for various uses).
        let empty: *mut Object = msg_send![class![NSString], alloc];
        let empty: *mut Object = msg_send![empty, init];
        let empty: *mut Object = msg_send![empty, autorelease];

        // Create the application menu bar.
        let appname: *mut Object = msg_send![class![NSString], alloc];
        let appname: *mut Object = msg_send![appname,
                                             initWithBytes: APP_NAME.as_ptr()
                                             length: APP_NAME.len()
                                             encoding: 4]; // UTF-8
        let appname: *mut Object = msg_send![appname, autorelease];

        let mainmenu: *mut Object = msg_send![class![NSMenu], alloc];
        let mainmenu: *mut Object = msg_send![mainmenu, initWithTitle: appname];
        let mainmenu: *mut Object = msg_send![mainmenu, autorelease];

        let appmenuitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let appmenuitem: *mut Object = msg_send![appmenuitem, init];
        let appmenuitem: *mut Object = msg_send![appmenuitem, autorelease];

        let _: () = msg_send![mainmenu, addItem: appmenuitem];
        let _: () = msg_send![app.app, setMainMenu: mainmenu];

        // "About atomCAD"
        let s = format!("About {}", APP_NAME);
        let aboutmsg: *mut Object = msg_send![class![NSString], alloc];
        let aboutmsg: *mut Object = msg_send![aboutmsg,
                                              initWithBytes: s.as_ptr()
                                              length: s.len()
                                              encoding: 4]; // UTF-8
        let aboutmsg: *mut Object = msg_send![aboutmsg, autorelease];

        let aboutitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let aboutitem: *mut Object = msg_send![aboutitem,
                                               initWithTitle: aboutmsg
                                               action: sel!(orderFrontStandardAboutPanel:)
                                               keyEquivalent: empty];
        let aboutitem: *mut Object = msg_send![aboutitem, autorelease];

        // "Settings... [⌘,]"
        let s = "Settings...";
        let settingsmsg: *mut Object = msg_send![class![NSString], alloc];
        let settingsmsg: *mut Object = msg_send![settingsmsg,
                                                 initWithBytes: s.as_ptr()
                                                 length: s.len()
                                                 encoding: 4]; // UTF-8
        let settingsmsg: *mut Object = msg_send![settingsmsg, autorelease];

        let s = ","; // ⌘-, shortcut
        let settingskey: *mut Object = msg_send![class![NSString], alloc];
        let settingskey: *mut Object = msg_send![settingskey,
                                                 initWithBytes: s.as_ptr()
                                                 length: s.len()
                                                 encoding: 4]; // UTF-8
        let settingskey: *mut Object = msg_send![settingskey, autorelease];

        let settingsitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let settingsitem: *mut Object = msg_send![settingsitem,
                                                  initWithTitle: settingsmsg
                                                  action: 0
                                                  keyEquivalent: settingskey];
        let settingsitem: *mut Object = msg_send![settingsitem, autorelease];

        // "Serives" menu
        let servicesmenu: *mut Object = msg_send![class![NSMenu], alloc];
        let servicesmenu: *mut Object = msg_send![servicesmenu, init];
        let servicesmenu: *mut Object = msg_send![servicesmenu, autorelease];
        let _: () = msg_send![app.app, setServicesMenu: servicesmenu];

        let s = "Services";
        let servicesmsg: *mut Object = msg_send![class![NSString], alloc];
        let servicesmsg: *mut Object = msg_send![servicesmsg,
                                                 initWithBytes: s.as_ptr()
                                                 length: s.len()
                                                 encoding: 4]; // UTF-8
        let servicesmsg: *mut Object = msg_send![servicesmsg, autorelease];

        let servicesitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let servicesitem: *mut Object = msg_send![servicesitem,
                                                  initWithTitle: servicesmsg
                                                  action: 0
                                                  keyEquivalent: empty];
        let servicesitem: *mut Object = msg_send![servicesitem, autorelease];
        let _: () = msg_send![servicesitem, setSubmenu: servicesmenu];

        // "Quit atomCAD [⌘Q]"
        let s = format!("Quit {}", APP_NAME);
        let quitmsg: *mut Object = msg_send![class![NSString], alloc];
        let quitmsg: *mut Object = msg_send![quitmsg,
                                             initWithBytes: s.as_ptr()
                                             length: s.len()
                                             encoding: 4]; // UTF-8
        let quitmsg: *mut Object = msg_send![quitmsg, autorelease];

        let s = "q"; // ⌘-q shortcut
        let quitkey: *mut Object = msg_send![class![NSString], alloc];
        let quitkey: *mut Object = msg_send![quitkey,
                                             initWithBytes: s.as_ptr()
                                             length: s.len()
                                             encoding: 4]; // UTF-8
        let quitkey: *mut Object = msg_send![quitkey, autorelease];

        let quititem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let quititem: *mut Object = msg_send![quititem,
                                              initWithTitle: quitmsg
                                              action: sel!(terminate:)
                                              keyEquivalent: quitkey];
        let quititem: *mut Object = msg_send![quititem, autorelease];

        // Create the “atomCAD” application menu.
        let atomcadmenu: *mut Object = msg_send![class![NSMenu], alloc];
        let atomcadmenu: *mut Object = msg_send![atomcadmenu, init];
        let atomcadmenu: *mut Object = msg_send![atomcadmenu, autorelease];

        // Add “atomCAD” application menu.
        let _: () = msg_send![atomcadmenu, addItem: aboutitem];
        let sep: *mut Object = msg_send![class![NSMenuItem], separatorItem];
        let _: () = msg_send![atomcadmenu, addItem: sep];
        let _: () = msg_send![atomcadmenu, addItem: settingsitem];
        let sep: *mut Object = msg_send![class![NSMenuItem], separatorItem];
        let _: () = msg_send![atomcadmenu, addItem: sep];
        let _: () = msg_send![atomcadmenu, addItem: servicesitem];
        let sep: *mut Object = msg_send![class![NSMenuItem], separatorItem];
        let _: () = msg_send![atomcadmenu, addItem: sep];
        let _: () = msg_send![atomcadmenu, addItem: quititem];
        let _: () = msg_send![appmenuitem, setSubmenu: atomcadmenu];
    });

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

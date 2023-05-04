// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use atomcad::platform::relaunch;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
use objc::rc::autoreleasepool;
#[cfg(target_os = "macos")]
use objc::runtime::Object;

use winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

fn replace_menu_bar(app_name: &str) {
    // Create the menu on macOS using Cocoa APIs.
    #[cfg(target_os = "macos")]
    autoreleasepool(|| unsafe {
        // Get the application object.
        let app: *mut Object = msg_send![class![NSApplication], sharedApplication];

        // Empty string (for various uses).
        let empty: *mut Object = msg_send![class![NSString], alloc];
        let empty: *mut Object = msg_send![empty, init];
        let empty: *mut Object = msg_send![empty, autorelease];

        // Create the application menu bar.
        let appname: *mut Object = msg_send![class![NSString], alloc];
        let appname: *mut Object = msg_send![appname,
                                             initWithBytes: app_name.as_ptr()
                                             length: app_name.len()
                                             encoding: 4]; // UTF-8
        let appname: *mut Object = msg_send![appname, autorelease];

        let mainmenu: *mut Object = msg_send![class![NSMenu], alloc];
        let mainmenu: *mut Object = msg_send![mainmenu, initWithTitle: appname];
        let mainmenu: *mut Object = msg_send![mainmenu, autorelease];

        let appmenuitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let appmenuitem: *mut Object = msg_send![appmenuitem, init];
        let appmenuitem: *mut Object = msg_send![appmenuitem, autorelease];

        let _: () = msg_send![mainmenu, addItem: appmenuitem];
        let _: () = msg_send![app, setMainMenu: mainmenu];

        // "About atomCAD"
        let s = format!("About {}", app_name);
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
        let _: () = msg_send![app, setServicesMenu: servicesmenu];

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

        // "Hide atomCAD [⌘H]"
        let s = format!("Hide {}", app_name);
        let hidemsg: *mut Object = msg_send![class![NSString], alloc];
        let hidemsg: *mut Object = msg_send![hidemsg,
                                             initWithBytes: s.as_ptr()
                                             length: s.len()
                                             encoding: 4]; // UTF-8
        let hidemsg: *mut Object = msg_send![hidemsg, autorelease];

        let s = "h"; // ⌘-h shortcut
        let hidekey: *mut Object = msg_send![class![NSString], alloc];
        let hidekey: *mut Object = msg_send![hidekey,
                                             initWithBytes: s.as_ptr()
                                             length: s.len()
                                             encoding: 4]; // UTF-8
        let hidekey: *mut Object = msg_send![hidekey, autorelease];

        let hideitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let hideitem: *mut Object = msg_send![hideitem,
                                              initWithTitle: hidemsg
                                              action: sel!(hide:)
                                              keyEquivalent: hidekey];
        let hideitem: *mut Object = msg_send![hideitem, autorelease];

        // "Hide Others [⌥⌘H]"
        let s = "Hide Others";
        let hideothersmsg: *mut Object = msg_send![class![NSString], alloc];
        let hideothersmsg: *mut Object = msg_send![hideothersmsg,
                                                   initWithBytes: s.as_ptr()
                                                   length: s.len()
                                                   encoding: 4]; // UTF-8
        let hideothersmsg: *mut Object = msg_send![hideothersmsg, autorelease];

        let s = "h"; // ⌘-h shortcut
        let hideotherskey: *mut Object = msg_send![class![NSString], alloc];
        let hideotherskey: *mut Object = msg_send![hideotherskey,
                                                   initWithBytes: s.as_ptr()
                                                   length: s.len()
                                                   encoding: 4]; // UTF-8
        let hideotherskey: *mut Object = msg_send![hideotherskey, autorelease];

        let hideothersitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let hideothersitem: *mut Object = msg_send![hideothersitem,
                                                    initWithTitle: hideothersmsg
                                                    action: sel!(hideOtherApplications:)
                                                    keyEquivalent: hideotherskey];
        let _: () = msg_send![hideothersitem, setKeyEquivalentModifierMask: 0x180000]; // ⌥⌘
        let hideothersitem: *mut Object = msg_send![hideothersitem, autorelease];

        // "Show All"
        let s = "Show All";
        let showallmsg: *mut Object = msg_send![class![NSString], alloc];
        let showallmsg: *mut Object = msg_send![showallmsg,
                                                initWithBytes: s.as_ptr()
                                                length: s.len()
                                                encoding: 4]; // UTF-8
        let showallmsg: *mut Object = msg_send![showallmsg, autorelease];

        let showallitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let showallitem: *mut Object = msg_send![showallitem,
                                                 initWithTitle: showallmsg
                                                 action: sel!(unhideAllApplications:)
                                                 keyEquivalent: empty];
        let showallitem: *mut Object = msg_send![showallitem, autorelease];

        // "Quit atomCAD [⌘Q]"
        let s = format!("Quit {}", app_name);
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
        let _: () = msg_send![atomcadmenu, addItem: hideitem];
        let _: () = msg_send![atomcadmenu, addItem: hideothersitem];
        let _: () = msg_send![atomcadmenu, addItem: showallitem];
        let sep: *mut Object = msg_send![class![NSMenuItem], separatorItem];
        let _: () = msg_send![atomcadmenu, addItem: sep];
        let _: () = msg_send![atomcadmenu, addItem: quititem];
        let _: () = msg_send![appmenuitem, setSubmenu: atomcadmenu];
    });
}

fn main() {
    const APP_NAME: &str = "atomCAD";

    // If we are running from the command line (e.g. as a result of `cargo
    // run`), relaunch as a dynamically created app bundle.  This currently
    // only has any effect on macOS, where it is required because many Cocoa
    // APIs will not work unless the application is running from a bundle.
    match relaunch::Trampoline::new(&APP_NAME, "io.atomcad.atomCAD", env!("CARGO_PKG_VERSION")) {
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
            Event::NewEvents(StartCause::Init) => {
                // Will be called once when the event loop starts.
                replace_menu_bar(APP_NAME);
            }
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

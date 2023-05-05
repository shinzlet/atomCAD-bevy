// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
    winit::{WinitSettings, WinitWindows},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use atomcad::platform::relaunch;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os = "macos")]
use objc::rc::autoreleasepool;
#[cfg(target_os = "macos")]
use objc::runtime::Object;

const APP_NAME: &str = "atomCAD";

fn nsstring(s: &str) -> *mut Object {
    unsafe {
        let cls = class!(NSString);
        let bytes = s.as_ptr();
        let len = s.len();
        let encoding = 4; // UTF-8
        let obj: *mut Object = msg_send![cls, alloc];
        let obj: *mut Object = msg_send![obj, initWithBytes:bytes length:len encoding:encoding];
        let obj: *mut Object = msg_send![obj, autorelease];
        obj
    }
}

fn replace_menu_bar(
    // We have to use `NonSend` here.  This forces this function to be called
    // from the winit thread (which is the main thread on macOS), after the
    // window has been created.  We don't actually use it, but this does
    // control when and from where we will be called.
    _windows: NonSend<WinitWindows>,
) {
    // Create the menu on macOS using Cocoa APIs.
    #[cfg(target_os = "macos")]
    autoreleasepool(|| unsafe {
        // Get the application object.
        let app: *mut Object = msg_send![class![NSApplication], sharedApplication];

        // Empty string (for various uses).
        let empty = nsstring("");

        // Create the application menu bar.
        let appname = nsstring(APP_NAME);

        let mainmenu: *mut Object = msg_send![class![NSMenu], alloc];
        let mainmenu: *mut Object = msg_send![mainmenu, initWithTitle: appname];
        let mainmenu: *mut Object = msg_send![mainmenu, autorelease];

        let appmenuitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let appmenuitem: *mut Object = msg_send![appmenuitem, init];
        let appmenuitem: *mut Object = msg_send![appmenuitem, autorelease];

        let _: () = msg_send![mainmenu, addItem: appmenuitem];
        let _: () = msg_send![app, setMainMenu: mainmenu];

        // "About atomCAD"
        let aboutmsg = nsstring(&format!("About {}", APP_NAME));

        let aboutitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let aboutitem: *mut Object = msg_send![aboutitem,
                                               initWithTitle: aboutmsg
                                               action: sel!(orderFrontStandardAboutPanel:)
                                               keyEquivalent: empty];
        let aboutitem: *mut Object = msg_send![aboutitem, autorelease];

        // "Settings... [⌘,]"
        let settingsmsg = nsstring("Settings...");
        let settingskey = nsstring(",");

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

        let servicesmsg = nsstring("Services");

        let servicesitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let servicesitem: *mut Object = msg_send![servicesitem,
                                                  initWithTitle: servicesmsg
                                                  action: 0
                                                  keyEquivalent: empty];
        let servicesitem: *mut Object = msg_send![servicesitem, autorelease];
        let _: () = msg_send![servicesitem, setSubmenu: servicesmenu];

        // "Hide atomCAD [⌘H]"
        let hidemsg = nsstring(&format!("Hide {}", APP_NAME));
        let hidekey = nsstring("h");

        let hideitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let hideitem: *mut Object = msg_send![hideitem,
                                              initWithTitle: hidemsg
                                              action: sel!(hide:)
                                              keyEquivalent: hidekey];
        let hideitem: *mut Object = msg_send![hideitem, autorelease];

        // "Hide Others [⌥⌘H]"
        let hideothersmsg = nsstring("Hide Others");
        let hideotherskey = nsstring("h");

        let hideothersitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let hideothersitem: *mut Object = msg_send![hideothersitem,
                                                    initWithTitle: hideothersmsg
                                                    action: sel!(hideOtherApplications:)
                                                    keyEquivalent: hideotherskey];
        let _: () = msg_send![hideothersitem, setKeyEquivalentModifierMask: 0x180000]; // ⌥⌘
        let hideothersitem: *mut Object = msg_send![hideothersitem, autorelease];

        // "Show All"
        let showallmsg = nsstring("Show All");

        let showallitem: *mut Object = msg_send![class![NSMenuItem], alloc];
        let showallitem: *mut Object = msg_send![showallitem,
                                                 initWithTitle: showallmsg
                                                 action: sel!(unhideAllApplications:)
                                                 keyEquivalent: empty];
        let showallitem: *mut Object = msg_send![showallitem, autorelease];

        // "Quit atomCAD [⌘Q]"
        let quitmsg = nsstring(&format!("Quit {}", APP_NAME));
        let quitkey = nsstring("q");

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

    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_NAME.into(),
                // Turn off vsync to maximize CPU/GPU usage
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_startup_system(replace_menu_bar)
        .add_startup_system(setup)
        .add_system(ui_hello_world)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // torus
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Torus {
            radius: 1.0,
            subdivisions_segments: 4,
            subdivisions_sides: 16,
            ..default()
        })),
        material: materials.add(Color::rgb(0.2, 0.8, 0.4).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, 4.0),
        ..default()
    });
}

fn ui_hello_world(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("Hello World!");
    });
}

// End of File

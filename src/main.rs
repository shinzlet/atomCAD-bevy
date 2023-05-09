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

#[cfg(target_os = "macos")]
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

// A menubar is a hierarchical list of actions with attached titles and/or
// keyboard shortcuts.  It is attached to either the application instance
// (macOS) or the main window (Windows/Linux).
//
// Menus can also be contextual (e.g. a popup right-click menu) or accessed
// from the system tray.
struct Menu {
    title: String,
    items: Vec<MenuItem>,
}

impl Menu {
    fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            items: Vec::new(),
        }
    }

    fn add(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }
}

// A menu item is either an action (with an optional keyboard shortcut) or a
// submenu.  The Separator is a visual divider between groups of related menu
// items.
enum MenuItem {
    Separator,
    Entry(String, MenuShortcut, MenuAction),
    SubMenu(Menu),
}

impl MenuItem {
    fn new(title: &str, shortcut: MenuShortcut, action: MenuAction) -> Self {
        Self::Entry(title.to_owned(), shortcut, action)
    }
}

// A keyboard shortcut is a combination of modifier keys (e.g. Shift, Option,
// Alt, etc.) and the key to press (indicated by a unicode character).
#[derive(Clone, Copy)]
enum MenuShortcut {
    None,
    System(SystemShortcut),
}

// Common actions like copy-paste, file-open, and quit are usually bound to
// shortcuts that vary from platform to platform, but are expected to remain
// consistent across all apps on that platform.
#[derive(Clone, Copy)]
enum SystemShortcut {
    Preferences,
    HideApp,
    HideOthers,
    QuitApp,
}

#[derive(Clone, Copy, PartialEq)]
struct ModifierKeys(u8);

impl ModifierKeys {
    const NONE: ModifierKeys = ModifierKeys(0);
    const CAPSLOCK: ModifierKeys = ModifierKeys(1 << 0);
    const SHIFT: ModifierKeys = ModifierKeys(1 << 1);
    const CONTROL: ModifierKeys = ModifierKeys(1 << 2);
    const OPTION: ModifierKeys = ModifierKeys(1 << 3);
    const COMMAND: ModifierKeys = ModifierKeys(1 << 4);
    const NUMPAD: ModifierKeys = ModifierKeys(1 << 5);
    const HELP: ModifierKeys = ModifierKeys(1 << 6);
    const FUNCTION: ModifierKeys = ModifierKeys(1 << 7);

    fn contains(self, other: ModifierKeys) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for ModifierKeys {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ModifierKeys(self.0 | rhs.0)
    }
}

// A menu action is a callback that is invoked when the menu item is selected.
// There are also a number of important platform-specific actions that can be
// invoked.
enum MenuAction {
    System(SystemAction),
}

enum SystemAction {
    LaunchAboutWindow,
    LaunchPreferences,
    ServicesMenu,
    HideApp,
    HideOthers,
    ShowAll,
    Terminate,
}

unsafe fn build_menu(app: *mut Object, services_menu: *mut Object, menu: &Menu) -> *mut Object {
    // Create root menu bar.
    let menuobj: *mut Object = msg_send![class![NSMenu], alloc];
    let menuobj: *mut Object = msg_send![menuobj, initWithTitle: nsstring(&menu.title)];
    let menuobj: *mut Object = msg_send![menuobj, autorelease];

    for menuitem in menu.items.iter() {
        match menuitem {
            MenuItem::Separator => {
                let item: *mut Object = msg_send![class![NSMenuItem], separatorItem];
                let _: () = msg_send![menuobj, addItem: item];
            }
            MenuItem::Entry(title, shortcut, action) => {
                let title = nsstring(&title);
                let mut is_service_menu = false;
                let action = match action {
                    MenuAction::System(action) => match action {
                        SystemAction::LaunchAboutWindow => {
                            Some(sel!(orderFrontStandardAboutPanel:))
                        }
                        SystemAction::LaunchPreferences => Some(sel!(orderFrontPreferencesPanel:)),
                        SystemAction::ServicesMenu => {
                            is_service_menu = true;
                            None
                        }
                        SystemAction::HideApp => Some(sel!(hide:)),
                        SystemAction::HideOthers => Some(sel!(hideOtherApplications:)),
                        SystemAction::ShowAll => Some(sel!(unhideAllApplications:)),
                        SystemAction::Terminate => Some(sel!(terminate:)),
                    },
                };
                let shortcutkey = match shortcut {
                    MenuShortcut::None => nsstring(""),
                    MenuShortcut::System(shortcut) => match shortcut {
                        SystemShortcut::Preferences => nsstring(","),
                        SystemShortcut::HideApp => nsstring("h"),
                        SystemShortcut::HideOthers => nsstring("h"),
                        SystemShortcut::QuitApp => nsstring("q"),
                    },
                };
                let shotcutmodifiers = match shortcut {
                    MenuShortcut::None => ModifierKeys::NONE,
                    MenuShortcut::System(shortcut) => match shortcut {
                        SystemShortcut::Preferences => ModifierKeys::COMMAND,
                        SystemShortcut::HideApp => ModifierKeys::COMMAND,
                        SystemShortcut::HideOthers => ModifierKeys::COMMAND | ModifierKeys::OPTION,
                        SystemShortcut::QuitApp => ModifierKeys::COMMAND,
                    },
                };
                let mut item: *mut Object = msg_send![class![NSMenuItem], alloc];
                if let Some(action) = action {
                    item = msg_send![item,
                                     initWithTitle: title
                                     action: action
                                     keyEquivalent: shortcutkey];
                } else {
                    item = msg_send![item,
                                     initWithTitle: title
                                     action: 0
                                     keyEquivalent: shortcutkey];
                }
                if shotcutmodifiers != ModifierKeys::NONE {
                    let mut modifiermask = 0usize;
                    if shotcutmodifiers.contains(ModifierKeys::CAPSLOCK) {
                        modifiermask |= 1 << 16; // NSEventModifierFlagCapsLock
                    }
                    if shotcutmodifiers.contains(ModifierKeys::SHIFT) {
                        modifiermask |= 1 << 17; // NSEventModifierFlagShift
                    }
                    if shotcutmodifiers.contains(ModifierKeys::CONTROL) {
                        modifiermask |= 1 << 18; // NSEventModifierFlagControl
                    }
                    if shotcutmodifiers.contains(ModifierKeys::OPTION) {
                        modifiermask |= 1 << 19; // NSEventModifierFlagOption
                    }
                    if shotcutmodifiers.contains(ModifierKeys::COMMAND) {
                        modifiermask |= 1 << 20; // NSEventModifierFlagCommand
                    }
                    if shotcutmodifiers.contains(ModifierKeys::NUMPAD) {
                        modifiermask |= 1 << 21; // NSEventModifierFlagNumericPad
                    }
                    if shotcutmodifiers.contains(ModifierKeys::HELP) {
                        modifiermask |= 1 << 22; // NSEventModifierFlagHelp
                    }
                    if shotcutmodifiers.contains(ModifierKeys::FUNCTION) {
                        modifiermask |= 1 << 23; // NSEventModifierFlagFunction
                    }
                    let _: () = msg_send![item, setKeyEquivalentModifierMask: modifiermask];
                }
                item = msg_send![item, autorelease];
                if is_service_menu {
                    let _: () = msg_send![item, setSubmenu: services_menu];
                }
                let _: () = msg_send![menuobj, addItem: item];
            }
            MenuItem::SubMenu(submenu) => {
                let item: *mut Object = msg_send![class![NSMenuItem], alloc];
                let item: *mut Object = msg_send![item, init];
                let item: *mut Object = msg_send![item, autorelease];
                let submenu = build_menu(app, services_menu, &submenu);
                let _: () = msg_send![item, setSubmenu: submenu];
                let _: () = msg_send![menuobj, addItem: item];
            }
        }
    }

    // Return the menu object to the caller.
    menuobj
}

#[cfg(target_os = "macos")]
fn attach_menu(
    // On some platforms, e.g. Windows and Linux, the menu bar is part of the
    // window itself, and we need to add it to each individual window.  But
    // for macOS the menu bar is a property of the NSApplication instance
    // shared by the entire process, so we only need to set it once and don't
    // use the `WinitWindows` parameter.
    _windows: &WinitWindows,
    menu: &Menu,
) {
    // Create the menu on macOS using Cocoa APIs.
    #[cfg(target_os = "macos")]
    autoreleasepool(|| unsafe {
        // Get the application object.
        let app: *mut Object = msg_send![class![NSApplication], sharedApplication];

        // Create and register the services menu.
        let services_menu: *mut Object = msg_send![class![NSMenu], alloc];
        let services_menu: *mut Object = msg_send![services_menu, init];
        let services_menu: *mut Object = msg_send![services_menu, autorelease];
        let _: () = msg_send![app, setServicesMenu: services_menu];

        // Turn the menubar description into a Cocoa menu.
        let obj = build_menu(app, services_menu, &menu);

        // Register the menu with the NSApplication object.
        let _: () = msg_send![app, setMainMenu: obj];
    });
}

fn replace_menu_bar(
    // We have to use `NonSend` here.  This forces this function to be called
    // from the winit thread (which is the main thread on macOS), after the
    // window has been created.  We don't actually use it on macOS, but this
    // does control when and from where we will be called.
    windows: NonSend<WinitWindows>,
) {
    let menubar = Menu::new(APP_NAME).add(MenuItem::SubMenu(
        Menu::new("")
            .add(MenuItem::new(
                &format!("About {}", APP_NAME),
                MenuShortcut::None,
                MenuAction::System(SystemAction::LaunchAboutWindow),
            ))
            .add(MenuItem::Separator)
            .add(MenuItem::new(
                "Settings...",
                MenuShortcut::System(SystemShortcut::Preferences),
                MenuAction::System(SystemAction::LaunchPreferences),
            ))
            .add(MenuItem::Separator)
            .add(MenuItem::new(
                "Services",
                MenuShortcut::None,
                MenuAction::System(SystemAction::ServicesMenu),
            ))
            .add(MenuItem::Separator)
            .add(MenuItem::new(
                &format!("Hide {}", APP_NAME),
                MenuShortcut::System(SystemShortcut::HideApp),
                MenuAction::System(SystemAction::HideApp),
            ))
            .add(MenuItem::new(
                "Hide Others",
                MenuShortcut::System(SystemShortcut::HideOthers),
                MenuAction::System(SystemAction::HideOthers),
            ))
            .add(MenuItem::new(
                "Show All",
                MenuShortcut::None,
                MenuAction::System(SystemAction::ShowAll),
            ))
            .add(MenuItem::Separator)
            .add(MenuItem::new(
                &format!("Quit {}", APP_NAME),
                MenuShortcut::System(SystemShortcut::QuitApp),
                MenuAction::System(SystemAction::Terminate),
            )),
    ));

    // Do the platform-dependent work of constructing the menubar and
    // attaching it to the application object or main window.
    attach_menu(&(*windows), &menubar);
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

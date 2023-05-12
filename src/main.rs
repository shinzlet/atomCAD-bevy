// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

use atomcad::menubar::winit_menu_bar;
use atomcad::platform::relaunch;
use atomcad::APP_NAME;

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
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(winit_menu_bar)
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
    // infinite grid
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -1.0, 0.0),
        ..Default::default()
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

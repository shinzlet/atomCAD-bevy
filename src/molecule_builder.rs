// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Plan of attack (having never used bevy before, so not sure if this is ideal):
// - there is a type of object called a molecule, which according to bevy ecs is just a u64 id
// - Each molecule represents a collection of atoms, including bonding information
// - clicking on a shown electron pair will produce a bond to it
// - I'm not worrying about resonance / multiple bonding for now because this is my first step

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
type Element = u32;
type BondOrder = u32;
type Molecule = petgraph::graph::UnGraph<Atom, BondOrder>;

#[derive(Default, Component)]
pub struct Atom {
    pos: Vec3,
    element: u32,
    entity: Option<Entity>
}

#[derive(Default, Component)]
pub struct BindingSite {
    pos: Vec3,
}

#[derive(Resource)]
pub struct AtomPbr {
    lone_pair: PbrBundle,
    carbon: PbrBundle
}

// All of the molecules in the current project
#[derive(Default, Resource)]
pub struct Workspace {
    molecules: Vec<Molecule>
}

#[derive(Default, Component)]
pub struct ClickFlag {}

pub fn molecule_builder(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BindingSite, &Transform), With<ClickFlag>>,
    atompbr: Res<AtomPbr>,
    mut workspace: ResMut<Workspace>
) {
    for (mut entity, mut site, transform) in query.iter_mut() {
        workspace.molecules[0].add_node(Atom::default());
        println!("gen {} idx {}", entity.generation(), entity.index());
        // Destroy the binding site
        commands.entity(entity).despawn();

        // Place a new atom
        let mut new_atom = atompbr.carbon.clone();
        new_atom.transform = *transform;
        let new_atom = commands.spawn(new_atom).id();

        // Add a new binding site
        let mut lone_pair = atompbr.lone_pair.clone();
        lone_pair.transform = *transform;
        lone_pair.transform.translation += Vec3::new(1.5, 0.0, 0.0);
        commands.spawn((
            lone_pair,
            RaycastPickTarget::default(),   // Marker for the `bevy_picking_raycast` backend
            OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
                target_commands.insert(ClickFlag::default());
            }),
            BindingSite::default()
        ));
    }
}

pub fn init_molecule(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let atompbr = AtomPbr {
        carbon: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                sectors: 8,
                stacks: 8
            })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        lone_pair: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.3,
                sectors: 8,
                stacks: 8
            })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    };

    // Create an initial carbon atom
    commands.spawn(atompbr.carbon.clone());

    // Create a lone pair
    let mut lone_pair = atompbr.lone_pair.clone();
    lone_pair.transform = Transform::from_xyz(1.5, 0.0, 0.0);
    commands.spawn((
        lone_pair,
        RaycastPickTarget::default(),   // Marker for the `bevy_picking_raycast` backend
        OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
            target_commands.insert(ClickFlag::default());
        }),
        BindingSite::default()
    ));

    commands.insert_resource(atompbr);

    let mut workspace = Workspace::default();
    workspace.molecules.push(Molecule::default());

    commands.insert_resource(workspace);
}

fn spawn_atom() {
}

// End of File

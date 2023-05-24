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
type Molecule = petgraph::graph::UnGraph<Vertex, BondOrder>;

#[derive(Component, Debug)]
pub struct Atom {
    pos: Vec3,
    element: u32,
    entity: Entity
}

impl Into<Vertex> for Atom {
    fn into(self) -> Vertex {
        Vertex::Atom(self)
    }
}

#[derive(Debug)]
pub struct LonePair {
    pos: Vec3
}

impl Into<Vertex> for LonePair {
    fn into(self) -> Vertex {
        Vertex::LonePair(self)
    }
}

#[derive(Debug)]
pub enum Vertex {
    Atom(Atom),
    LonePair(LonePair)
}

//impl Vertex {
    //fn new(atom: Atom) {
        //Vertex::Atom(atom)
    //}

    //fn new(lone_pair: LonePair) {
        //Vertex::LonePair(lone_pair)
    //}
//}

// Must have only one bond in the graph, which connects it to
// the target atom
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
        // workspace.molecules[0].add_node(Atom::default());
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
    let carbon_pbr = atompbr.carbon.clone();
    let initial_carbon_pos = carbon_pbr.transform.translation;
    let initial_carbon = commands.spawn(carbon_pbr).id();

    // Create a lone pair
    let mut initial_lone_pair_pbr = atompbr.lone_pair.clone();
    let initial_lone_pair_pos = Vec3::new(1.5, 0.0, 0.0);
    initial_lone_pair_pbr.transform.translation = initial_lone_pair_pos;
    let initial_lone_pair = commands.spawn((
        initial_lone_pair_pbr,
        RaycastPickTarget::default(),   // Marker for the `bevy_picking_raycast` backend
        OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
            target_commands.insert(ClickFlag::default());
        }),
        BindingSite::default()
    )).id();

    commands.insert_resource(atompbr);

    let mut workspace = Workspace::default();
    let mut initial_molecule = Molecule::default();

    let initial_carbon = Atom {
        entity: initial_carbon,
        pos: initial_carbon_pos,
        element: 6
    };

    let initial_lone_pair = LonePair {
        pos: initial_lone_pair_pos
    };

    let i1 = initial_molecule.add_node(initial_carbon.into());
    let i2 = initial_molecule.add_node(initial_lone_pair.into());
    initial_molecule.add_edge(i1, i2, 1);
    println!("{:?}", initial_molecule);
    workspace.molecules.push(initial_molecule);

    commands.insert_resource(workspace);
}

fn spawn_atom() {
}

// End of File

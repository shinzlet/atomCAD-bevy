// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Plan of attack (having never used bevy before, so not sure if this is ideal):
// - there is a type of object called a molecule, which according to bevy ecs is
// just a u64 id
//
// - Each molecule represents a collection of atoms, including bonding
// information
//
// - clicking on a shown electron pair will produce a bond to it
//
// - I'm not worrying about resonance / multiple bonding for now because this is
// my first step

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use petgraph::graph::NodeIndex;

type Element = u32;
type BondOrder = u32;
type MolGraph = petgraph::graph::UnGraph<Entity, BondOrder>;

#[derive(Component)]
pub struct Molecule {
    graph: MolGraph,
}

#[derive(Component, Debug)]
pub struct Atom {
    // I initially included the position here for serialization purposes:
    // you should be able to describe an atom's position without creating an
    // entity with a transform for each atom. However, this introduces a
    // data consistency problem - the entity transform and the atom position
    // are conflicting and would need to be kept synchronous. For now,
    // I'm avoiding the problem by only storing position inside of the
    // entity transform. This is bad long term though, as we probably want
    // double precision (not single float Vec3) relative to the COM (not
    // relative to the world origin).
    // pos: Vec3,
    element: u32,
}

//impl Into<Vertex> for Atom {
//fn into(self) -> Vertex {
//Vertex::Atom(self)
//}
//}

// Must have only one bond in the graph, which connects it to
// the target atom
#[derive(Component, Debug)]
pub struct LonePair {
    // See `Atom` to know why this is commented
    // pos: Vec3
}

//impl Into<Vertex> for LonePair {
//fn into(self) -> Vertex {
//Vertex::LonePair(self)
//}
//}

//#[derive(Debug)]
//pub enum Vertex {
//Atom(Atom),
//LonePair(LonePair),
//}

#[derive(Resource)]
pub struct AtomPbr {
    lone_pair: PbrBundle,
    carbon: PbrBundle,
}

#[derive(Default, Component)]
pub struct ClickFlag {}

pub fn molecule_builder(
    mut commands: Commands,
    mut query: Query<(Entity, &Parent, &Transform), With<ClickFlag>>,
    mut q_parent: Query<&mut Molecule>,
    atompbr: Res<AtomPbr>,
) {
    for (mut entity, mut parent, transform) in query.iter_mut() {
        // Retrieve the parent of the clicked particle - i.e. the
        // molecule graph.
        let molecule: &mut Molecule = q_parent.get_mut(parent.get()).unwrap().into_inner();
        println!("{:?}", molecule.graph);

        // Destroy the binding site:
        // First, find the node index in the molecule
        let index = molecule.graph.raw_nodes().iter().position(|n| n.weight == entity);
        let index = match index {
            None => {
                println!("couldn't remove!");
                // Give up on this click flag
                commands.entity(entity).remove::<ClickFlag>();
                return
            },
            Some(node_index) => NodeIndex::new(node_index)
        };

        // Get the atom this lone pair was connected to before removing it
        // (recall that we demand that all lone pairs have exactly one
        // neighbor)
        let bond_target = molecule.graph.neighbors(index).next().unwrap();

        molecule.graph.remove_node(index);
        commands.entity(entity).despawn();

        // Place a new atom
        let mut new_atom = atompbr.carbon.clone();
        new_atom.transform = *transform;
        let new_atom = commands.spawn((new_atom, Atom { element: 6 })).id();

        // Add a new binding site
        let mut lone_pair = atompbr.lone_pair.clone();
        lone_pair.transform = *transform;
        lone_pair.transform.translation += Vec3::new(1.5, 0.0, 0.0);
        let lone_pair = commands
            .spawn((
                lone_pair,
                RaycastPickTarget::default(),
                OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
                    target_commands.insert(ClickFlag::default());
                }),
                LonePair {},
            ))
            .id();

        // Store the graph indexes needed
        let new_atom_index = molecule.graph.add_node(new_atom);
        let lone_pair_index = molecule.graph.add_node(lone_pair);

        // Add a single bond between the atom and new lone pair
        molecule.graph.add_edge(new_atom_index, lone_pair_index, 1);

        // Add a single bond between the old atom and this atom:
        molecule.graph.add_edge(new_atom_index, bond_target, 1);

        commands.entity(parent.get()).push_children(&[new_atom, lone_pair]);
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
                stacks: 8,
            })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        lone_pair: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.3,
                sectors: 8,
                stacks: 8,
            })),
            material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    };

    // Create an initial carbon atom
    let carbon_pbr = atompbr.carbon.clone();
    let initial_carbon_pos = carbon_pbr.transform.translation;
    let initial_carbon = commands.spawn((carbon_pbr, Atom { element: 6 })).id();

    // Create a lone pair
    let mut initial_lone_pair_pbr = atompbr.lone_pair.clone();
    let initial_lone_pair_pos = Vec3::new(1.5, 0.0, 0.0);
    initial_lone_pair_pbr.transform.translation = initial_lone_pair_pos;
    let initial_lone_pair = commands
        .spawn((
            initial_lone_pair_pbr,
            RaycastPickTarget::default(),
            OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
                target_commands.insert(ClickFlag::default());
            }),
            LonePair {},
        ))
        .id();

    commands.insert_resource(atompbr);

    // Build the test molecule's graph
    let mut molgraph = MolGraph::default();

    // Store the graph indexes needed
    let i1 = molgraph.add_node(initial_carbon);
    let i2 = molgraph.add_node(initial_lone_pair);

    // Add a single bond between them
    molgraph.add_edge(i1, i2, 1);
    println!("{:?}", molgraph);

    // Create a molecule entity backed by the molecule graph - this
    // will allow us to use the ECS as a molecule database and give us
    // unique identifiers for each molecule
    let mut molecule = commands.spawn((
        Molecule { graph: molgraph },
        // A Visibility and ComputedVisibility are needed to make
        // the children of the molecule (the atoms) render. A transform
        // and global transform are needed for the child entities to
        // position themselves. There might be a name for this bundle -
        // in Godot, it would be something like `Spatial`
        Visibility::default(),
        ComputedVisibility::default(),
        GlobalTransform::default(),
        Transform::default(),
    ));

    // Make the displayed atom gameobjects children of the molecule, allowing
    // the molecule to be recovered when an atom is picked
    molecule.push_children(&[initial_carbon, initial_lone_pair]);
}

fn spawn_atom() {}

// End of File

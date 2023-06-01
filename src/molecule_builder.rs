// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use periodic_table::Element;
use petgraph::stable_graph::NodeIndex;
use std::collections::HashMap;

type BondOrder = u8;

/// Describes how different particles in a spawned molecule (including unbonded
/// electrons and atoms) are connected using a stable undirected graph. The
/// molecule must be spawned, because the graph's nodes are bevy `Entity`
/// instances, and all of their data (element, position, etc) is stored in the
/// entity's components. The edge weights represent integer bond order (1
/// indicates a single bond and so on). If a node of the molecule graph is a
/// `LonePair` Entity, it must have exactly one bond, and that bond must be to
/// an `Atom` Entity.
type MolGraph = petgraph::stable_graph::StableUnGraph<Entity, BondOrder>;

/// Stores a molecule graph as a component so that molecules can be stored in
/// ECS. This effectively allows us to use the ECS as a molecule workspace.
#[derive(Component)]
pub struct Molecule {
    graph: MolGraph,
}

/// The presence of this component means that an `Entity` models an atom in a
/// molecule.
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
    #[allow(dead_code)]
    element: Element
}

/// The presence of this component means that an `Entity` models a lone pair in
/// a molecule.
#[derive(Component, Debug)]
pub struct LonePair {
    // See `Atom` to know why this is commented
    // pos: Vec3
    node_index: NodeIndex,
}

/// Stores PbrBundles that are often duplicated, namely for things like atoms
/// and lone pairs. Note that cloning a PbrBundle only clones a `Handle` of the
/// Mesh and Material, so it is very cheap to clone this struct's members when
/// you need ownership.
/// TODO: this may be redundant - I think `Assets` serves a very similar purpose
#[derive(Resource)]
pub struct PbrCache {
    lone_pair: PbrBundle,
    atoms: HashMap<Element, PbrBundle>,
}

/// The `ClickFlag` indicates that the user clicked on an `Entity`, and that
/// this event is yet to be handled. Remove this flag if your system handled the
/// click.
#[derive(Default, Component)]
pub struct ClickFlag {}

/// Implements a simple molecule builder system that allows users to click lone
/// pairs to create new atoms and bonds. Supports multiple molecules in the same
/// world, because each `Molecule` is stored in ECS as a separate entity.
pub fn molecule_builder(
    mut commands: Commands,
    mut query: Query<(Entity, &Parent, &Transform, &LonePair), With<ClickFlag>>,
    mut q_parent: Query<&mut Molecule>,
    pbr_cache: Res<PbrCache>,
) {
    for (entity, parent, transform, clicked_lone_pair) in query.iter_mut() {
        // Retrieve the parent of the clicked particle - i.e. the
        // molecule graph.
        let molecule: &mut Molecule = q_parent.get_mut(parent.get()).unwrap().into_inner();

        let clicked_index = clicked_lone_pair.node_index;

        // Get the atom this lone pair was connected to before removing it
        // (recall that we demand that all lone pairs have exactly one
        // neighbor)
        let bond_target = molecule.graph.neighbors(clicked_index).next().unwrap();

        molecule.graph.remove_node(clicked_index);
        commands.entity(entity).despawn();

        // Place a new atom
        let mut new_atom = pbr_cache.atoms[&Element::Carbon].clone();
        new_atom.transform = *transform;
        let new_atom = commands
            .spawn((
                new_atom,
                Atom {
                    element: Element::Carbon,
                },
            ))
            .id();

        // Add a new binding site
        let mut lone_pair = pbr_cache.lone_pair.clone();
        lone_pair.transform = *transform;
        lone_pair.transform.translation += Vec3::new(1.5, 0.0, 0.0);
        let lone_pair = commands
            .spawn((
                lone_pair,
                RaycastPickTarget::default(),
                OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
                    target_commands.insert(ClickFlag::default());
                }),
            ))
            .id();

        // Store the graph indexes needed
        let new_atom_index = molecule.graph.add_node(new_atom);
        let lone_pair_index = molecule.graph.add_node(lone_pair);

        // Add a LonePair component to the entity so that it can track this
        commands.entity(lone_pair).insert(LonePair {
            node_index: lone_pair_index,
        });

        // Add a single bond between the atom and new lone pair
        molecule.graph.add_edge(new_atom_index, lone_pair_index, 1);

        // Add a single bond between the old atom and this atom:
        molecule.graph.add_edge(new_atom_index, bond_target, 1);

        commands
            .entity(parent.get())
            .push_children(&[new_atom, lone_pair]);

        println!("{:?}", molecule.graph);
    }
}

pub fn init_molecule(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut pbr_cache = PbrCache {
        atoms: HashMap::new(),
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

    pbr_cache.atoms.insert(
        Element::Carbon,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                sectors: 8,
                stacks: 8,
            })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    );

    // Create an initial carbon atom
    let carbon_pbr = pbr_cache.atoms[&Element::Carbon].clone();
    let initial_carbon = commands
        .spawn((
            carbon_pbr,
            Atom {
                element: Element::Carbon,
            },
        ))
        .id();

    // Create a lone pair
    let mut initial_lone_pair_pbr = pbr_cache.lone_pair.clone();
    initial_lone_pair_pbr.transform.translation = Vec3::new(1.5, 0.0, 0.0);
    let initial_lone_pair = commands
        .spawn((
            initial_lone_pair_pbr,
            RaycastPickTarget::default(),
            OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
                target_commands.insert(ClickFlag::default());
            }),
        ))
        .id();

    commands.insert_resource(pbr_cache);

    // Build the test molecule's graph
    let mut molgraph = MolGraph::default();

    // Store the graph indexes needed
    let i1 = molgraph.add_node(initial_carbon);
    let i2 = molgraph.add_node(initial_lone_pair);

    // Add a LonePair component to the entity so that it can track this
    commands
        .entity(initial_lone_pair)
        .insert(LonePair { node_index: i2 });

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

// End of File

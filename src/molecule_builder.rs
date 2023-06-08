// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vsepr::BOND_SHAPES;
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
/// `BondingSite` Entity, it must have exactly one bond, and that bond must be to
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
    element: Element,
}

/// The presence of this component means that an `Entity` models a bonding site in
/// a molecule.
#[derive(Component, Debug)]
pub struct BondingSite {
    // See `Atom` to know why this is commented
    // pos: Vec3
    node_index: NodeIndex,
}

/// Stores PbrBundles that are often duplicated, namely for things like atoms
/// and bonding sites. Note that cloning a PbrBundle only clones a `Handle` of the
/// Mesh and Material, so it is very cheap to clone this struct's members when
/// you need ownership.
/// TODO: this may be redundant - I think `Assets` serves a very similar purpose
#[derive(Resource)]
pub struct PbrCache {
    bonding_site: PbrBundle,
    atoms: HashMap<Element, PbrBundle>,
}

/// The `ClickFlag` indicates that the user clicked on an `Entity`, and that
/// this event is yet to be handled. Remove this flag if your system handled the
/// click.
#[derive(Default, Component)]
pub struct ClickFlag {}

/// Implements a simple molecule builder system that allows users to click
/// bonding sites to create new atoms and bonds. Supports multiple molecules in
/// the same world, because each `Molecule` is stored in ECS as a separate
/// entity.
pub fn molecule_builder(
    mut commands: Commands,
    mut query: Query<(Entity, &Parent, &Transform, &BondingSite), With<ClickFlag>>,
    mut q_parent: Query<&mut Molecule>,
    q_pos: Query<&Transform>,
    pbr_cache: Res<PbrCache>,
) {
    let pbr_cache = pbr_cache.into_inner();
    for (entity, parent, transform, clicked_bonding_site) in query.iter_mut() {
        // Retrieve the parent of the clicked particle - i.e. its molecule
        let molecule: &mut Molecule = q_parent.get_mut(parent.get()).unwrap().into_inner();

        let clicked_index = clicked_bonding_site.node_index;

        // // Get the atom this bonding site was connected to before removing it
        // // (recall that we demand that all bonding sites have exactly one
        // // neighbor)
        let bond_target = molecule.graph.neighbors(clicked_index).next().unwrap();
        molecule.graph.remove_node(clicked_index);
        commands.entity(entity).despawn();

        // The bonding sites are displayed quite close to the atom - because the
        // atoms are larger, we extend this displacement and spawn the new atom further
        // than the bonding site was located from its parent
        let parent_tf = q_pos
            .get(*molecule.graph.node_weight(bond_target).unwrap())
            .unwrap();
        let displacement = transform.translation - parent_tf.translation;
        let new_atom_pos = transform.translation + displacement;

        // We want the +z axis of this new atom to point from its center towards the atom it's bonded
        // to
        let up = -displacement.normalize();

        spawn_atom(
            &mut commands,
            parent.get(),
            &mut molecule.graph,
            &pbr_cache,
            false,
            new_atom_pos,
            up,
        );

        // // Place a new atom
        // let mut new_atom = pbr_cache.atoms[&Element::Carbon].clone();
        // new_atom.transform = *transform;
        // let new_atom = commands
        //     .spawn((
        //         new_atom,
        //         Atom {
        //             element: Element::Carbon,
        //         },
        //     ))
        //     .id();

        // // Add a new binding site
        // let mut bonding_site = pbr_cache.bonding_site.clone();
        // bonding_site.transform = *transform;
        // bonding_site.transform.translation += Vec3::new(1.5, 0.0, 0.0);
        // let bonding_site = commands
        //     .spawn((
        //         bonding_site,
        //         RaycastPickTarget::default(),
        //         OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
        //             target_commands.insert(ClickFlag::default());
        //         }),
        //     ))
        //     .id();

        // // Store the graph indexes needed
        // let new_atom_index = molecule.graph.add_node(new_atom);
        // let bonding_site_index = molecule.graph.add_node(bonding_site);

        // // Add a BondingSite component to the entity so that it can track this
        // commands.entity(bonding_site).insert(BondingSite {
        //     node_index: bonding_site_index,
        // });

        // // Add a single bond between the atom and new bonding site
        // molecule
        //     .graph
        //     .add_edge(new_atom_index, bonding_site_index, 1);

        // // Add a single bond between the old atom and this atom:
        // molecule.graph.add_edge(new_atom_index, bond_target, 1);

        // commands
        //     .entity(parent.get())
        //     .push_children(&[new_atom, bonding_site]);

        // println!("{:?}", molecule.graph);
    }
}

pub fn init_molecule(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut pbr_cache = PbrCache {
        atoms: HashMap::new(),
        bonding_site: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.3,
                sectors: 14,
                stacks: 14,
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
                sectors: 14,
                stacks: 14,
            })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    );

    // Build the test molecule's graph
    let mut molgraph = MolGraph::default();

    // Create a molecule entity backed by the molecule graph - this
    // will allow us to use the ECS as a molecule database and give us
    // unique identifiers for each molecule
    let mut molecule = commands.spawn((
        // Molecule { graph: molgraph },
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

    let molecule_id = molecule.id();
    spawn_atom(
        molecule.commands(),
        molecule_id,
        &mut molgraph,
        &pbr_cache,
        true,
        Vec3::default(),
        Vec3::new(0.0, 0.0, 1.0),
    );

    molecule.insert(Molecule { graph: molgraph });

    // Give ownership of the pbr cache to the ECS
    commands.insert_resource(pbr_cache);
}

// End of File
fn spawn_atom(
    commands: &mut Commands,
    molecule: Entity,
    molgraph: &mut MolGraph,
    pbr_cache: &PbrCache,
    skip_first_bonding_site: bool,
    position: Vec3,
    up: Vec3,
) {
    // Create a quaternion that will rotate from the global +z vector to the
    // up vector
    let up_rotation = Quat::from_rotation_arc(Vec3::new(0.0, 0.0, 1.0), up);

    // Create an initial carbon atom
    let mut carbon_pbr = pbr_cache.atoms[&Element::Carbon].clone();
    carbon_pbr.transform.translation = position;
    let initial_carbon = commands
        .spawn((
            carbon_pbr,
            Atom {
                element: Element::Carbon,
            },
        ))
        .id();
    let carbon_node = molgraph.add_node(initial_carbon);

    // Make the displayed particles gameobjects children of the molecule, allowing
    // the molecule to be recovered when a particle is picked
    commands.entity(molecule).add_child(initial_carbon);

    // Create bonding sites
    let mut angle_iter = BOND_SHAPES[5].unwrap().into_iter();
    if skip_first_bonding_site {
        angle_iter.next();
    }

    for angles in angle_iter {
        let mut bonding_site_pbr = pbr_cache.bonding_site.clone();
        let mut displacement = 1.0
            * Vec3 {
                x: angles.azimuthal.cos() * angles.polar.sin(),
                y: angles.azimuthal.sin() * angles.polar.sin(),
                z: angles.polar.cos(),
            };
        displacement = up_rotation * displacement;
        bonding_site_pbr.transform.translation = position + displacement;

        let bonding_site = commands
            .spawn((
                bonding_site_pbr,
                RaycastPickTarget::default(),
                OnPointer::<Click>::target_commands_mut(|_click, target_commands| {
                    target_commands.insert(ClickFlag::default());
                }),
            ))
            .id();

        // Store the graph indexes needed
        let bonding_site_node = molgraph.add_node(bonding_site);
        molgraph.add_edge(carbon_node, bonding_site_node, 1);

        // Add a BondingSite component to the entity so that it can track this
        commands.entity(bonding_site).insert(BondingSite {
            node_index: bonding_site_node,
        });

        commands.entity(molecule).add_child(bonding_site);
    }
}

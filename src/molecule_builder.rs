// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::vsepr::BOND_SHAPES;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use periodic_table::Element;
use petgraph::stable_graph::NodeIndex;
use petgraph::visit::IntoNeighbors;
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
type MolGraph = petgraph::stable_graph::StableUnGraph<MolNode, BondOrder>;

#[derive(Debug)]
struct MolNode {
    pos: Vec3,
    vel: Vec3,
    particle: Particle,
    id: Entity,
}
#[derive(Debug)]
enum Particle {
    Atom(Atom),
    BondingSite,
}

// A particle entity whose position should track a node from the molecule graph.
#[derive(Component)]
pub struct TrackedParticle {
    node_index: NodeIndex,
}

/// Stores a molecule graph as a component so that molecules can be stored in
/// ECS. This effectively allows us to use the ECS as a molecule workspace.
#[derive(Component)]
pub struct Molecule {
    graph: MolGraph,
}

/// The presence of this component means that an `Entity` models an atom in a
/// molecule.
#[derive(Debug)]
pub struct Atom {
    element: Element,
    // The NodeIndex of the atom that this Atom points towards. If None,
    // this atom's +z axis is aligned with the molecule's +z axis. If Some, the
    // +z axis of this atom points from the atom's center to the center of the
    // atom it is facing.
    facing: Option<NodeIndex>,
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
                radius: 0.5,
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
        false,
        Vec3::default(),
        Vec3::new(0.0, 0.0, 1.0),
        None,
    );

    molecule.insert(Molecule { graph: molgraph });

    // Give ownership of the pbr cache to the ECS
    commands.insert_resource(pbr_cache);
}
use std::time::{Duration, SystemTime};

pub fn track_particles(
    mut q_particle: Query<(&TrackedParticle, &mut Transform, &Parent)>,
    q_molecule: Query<&Molecule>,
) {
    for (particle, mut transform, parent) in q_particle.iter_mut() {
        let molecule = match q_molecule.get(parent.get()) {
            Ok(molecule) => molecule,
            Err(_) => {
                println!("An entity ({:?}) whose direct child uses a TrackedParticle component was not assigned to a Molecule!", parent.get());
                continue;
            }
        };

        if let Some(mol_node) = molecule.graph.node_weight(particle.node_index) {
            transform.translation = mol_node.pos;
        } else {
            println!("A TrackedParticle belonging to parent Entity {:?} had a nonexistent node index ({:?})", parent.get(), particle.node_index);
        }
    }
}

pub fn relax(
    mut commands: Commands,
    mut q_molecule: Query<&mut Molecule>,
    mut lines: ResMut<DebugLines>,
) {
    for mut molecule in q_molecule.iter_mut() {
        let mut graph = &mut molecule.graph;

        let mut forces = HashMap::<NodeIndex, Vec3>::new();

        for node_index in graph.node_indices() {
            let node = graph.node_weight(node_index).unwrap();
            if true {
                let mut force = Vec3::ZERO;
                // for neighbour_index in graph.neighbors(node_index) {
                //     let neighbour = graph.node_weight(neighbour_index).unwrap();
                //     if let Particle::Atom(_) = neighbour.particle {
                //         let displacement = neighbour.pos - node.pos;
                //         let force_str = displacement.length() - 1;
                //         force += displacement.normalize() * force_str;
                //     }
                // }

                for other_index in graph.node_indices() {
                    if other_index == node_index {
                        continue;
                    }

                    let other = graph.node_weight(other_index).unwrap();
                    if true {
                        let displacement = other.pos - node.pos;
                        if graph.contains_edge(node_index, other_index) {
                            let force_str = 2.0 * (displacement.length() - 1.0);
                            force += displacement.normalize() * force_str;
                        } else {
                            let force_str = displacement.length_recip().powi(2);
                            force += -displacement.normalize() * force_str;
                        }
                    }
                }

                forces.insert(node_index, force);
            }
        }

        for (node_index, force) in forces {
            graph.node_weight_mut(node_index).unwrap().vel += force * 0.1;
        }

        for mut node in graph.node_weights_mut() {
            node.pos += node.vel * 0.01;
            node.vel *= 0.9;
        }

        for edge in graph.edge_indices() {
            if let Some((a, b)) = graph.edge_endpoints(edge) {
                lines.line(
                    graph.node_weight(a).unwrap().pos,
                    graph.node_weight(b).unwrap().pos,
                    0.0,
                );
            }
        }
    }
}

fn on_bonding_site_clicked(
    In(click): In<ListenedEvent<Click>>,
    mut commands: Commands,
    mut q_clicked: Query<(&Parent, &Transform, &TrackedParticle)>,
    mut q_molecule: Query<&mut Molecule>,
    q_transform: Query<&Transform>,
    pbr_cache: Res<PbrCache>,
) -> Bubble {
    if let Ok((parent, transform, clicked_bonding_site)) = q_clicked.get(click.target) {
        // let pbr_cache = pbr_cache.into_inner();
        // Retrieve the parent of the clicked particle - i.e. its molecule
        let molecule: &mut Molecule = q_molecule.get_mut(parent.get()).unwrap().into_inner();

        let clicked_index = clicked_bonding_site.node_index;

        // Get the atom this bonding site was connected to before removing it
        // (recall that we demand that all bonding sites have exactly one
        // neighbor)
        let bond_target = molecule.graph.neighbors(clicked_index).next().unwrap();
        molecule.graph.remove_node(clicked_index);
        commands.entity(click.target).despawn();

        // The bonding sites are displayed quite close to the atom - because the
        // atoms are larger, we extend this displacement and spawn the new atom further
        // than the bonding site was located from its parent
        let parent_tf = q_transform
            .get(molecule.graph.node_weight(bond_target).unwrap().id)
            .unwrap();
        let displacement = transform.translation - parent_tf.translation;
        let new_atom_pos = transform.translation + displacement.normalize() * 0.5;

        // We want the +z axis of this new atom to point from its center towards the atom it's bonded
        // to
        let up = -displacement.normalize();

        let atom_node = spawn_atom(
            &mut commands,
            parent.get(),
            &mut molecule.graph,
            &pbr_cache.into_inner(),
            true,
            new_atom_pos,
            up,
            Some(bond_target),
        );

        // Add a single bond between the old atom and this atom:
        molecule.graph.add_edge(atom_node, bond_target, 1);
        println!("{:?}", molecule.graph);

        return Bubble::Burst;
    }

    Bubble::Up
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
    facing: Option<NodeIndex>,
) -> NodeIndex {
    // Create a quaternion that will rotate from the global +z vector to the
    // up vector
    let up_rotation = Quat::from_rotation_arc(Vec3::new(0.0, 0.0, 1.0), up);

    // Create an initial carbon atom
    let mut carbon_pbr = pbr_cache.atoms[&Element::Carbon].clone();
    carbon_pbr.transform.translation = position;
    let mut initial_carbon = commands.spawn((carbon_pbr));
    let carbon_node = molgraph.add_node(MolNode {
        pos: position,
        vel: Vec3::ZERO,
        particle: Particle::Atom(Atom {
            element: Element::Carbon,
            facing,
        }),
        id: initial_carbon.id(),
    });
    initial_carbon.insert(TrackedParticle {
        node_index: carbon_node,
    });
    let initial_carbon = initial_carbon.id();

    // Make the displayed particles gameobjects children of the molecule, allowing
    // the molecule to be recovered when a particle is picked
    commands.entity(molecule).add_child(initial_carbon);

    let num = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        % (BOND_SHAPES.len() - 1) as u128
        + 1;
    // Create bonding sites
    let mut angle_iter = BOND_SHAPES[num as usize].unwrap().into_iter();
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
                OnPointer::<Click>::run_callback(on_bonding_site_clicked),
            ))
            .id();

        // Store the graph indexes needed
        let bonding_site_node = molgraph.add_node(MolNode {
            pos: position + displacement,
            vel: Vec3::ZERO,
            particle: Particle::BondingSite,
            id: bonding_site,
        });
        molgraph.add_edge(carbon_node, bonding_site_node, 1);

        // Add a BondingSite component to the entity so that it can track this
        commands.entity(bonding_site).insert(TrackedParticle {
            node_index: bonding_site_node,
        });

        commands.entity(molecule).add_child(bonding_site);
    }

    carbon_node
}

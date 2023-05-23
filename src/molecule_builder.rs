// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Plan of attack (having never used bevy before, so not sure if this is ideal):
// - there is a type of object called a molecule, which according to bevy ecs is just a u64 id
// - Each molecule represents a collection of atoms, including bonding information
// - clicking on a shown electron pair will produce a bond to it
// - I'm not worrying about resonance / multiple bonding for now because this is my first step

use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Molecule {
    origin: Vec3,
    atoms: Vec<Atom>,
}

#[derive(Default, Component)]
pub struct Atom {
    pos: Vec3,
}

#[derive(Default, Component)]
pub struct BindingSite {
    pos: Vec3,
}

#[derive(Resource)]
pub struct AtomPbr {
    carbon: PbrBundle
}

#[derive(Default, Component)]
pub struct ClickFlag {}

pub fn molecule_builder(
    mut commands: Commands,
    // mut query: Query<(Entity, &mut Molecule), With<ClickFlag>>,
    mut query: Query<(Entity, &mut BindingSite, &Transform), With<ClickFlag>>,
    atompbr: Res<AtomPbr>
) {
    for (mut entity, mut site, transform) in query.iter_mut() {
        commands.entity(entity).despawn();
        let mut new_atom = atompbr.carbon.clone();
        new_atom.transform = *transform;
        commands.spawn(new_atom);
    }
}

pub fn init_molecule(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AtomPbr {
        carbon: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                sectors: 8,
                stacks: 8
            })),
            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    });
}

// End of File

use bevy::prelude::{Assets, BuildChildren, Color, Commands, default, Input, KeyCode, Mesh, PbrBundle, Query, Res, ResMut, shape, StandardMaterial, Transform, Vec2};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, ExternalForce, RigidBody, Velocity};
use crate::games::{LocalPlayer, Ship};

pub fn ship_input(mut local_player_ship: Query<&mut Ship, &LocalPlayer>, keyboard_input: Res<Input<KeyCode>>){
    local_player_ship.iter_mut().for_each(|mut ship| {
        ship.reset_inputs();
        if keyboard_input.pressed(KeyCode::Left) {
            ship.left_input = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            ship.right_input = true;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            ship.shoot_input = true;
        }
    });
}
pub fn ship_control(mut ships : Query<(&Ship, &Transform, &mut ExternalForce)>){
    //for mut ship in ships.

    ships.iter_mut().for_each(|(ship, transform, mut external_force)| {
        let position = transform.translation;
        let position2d = Vec2::new(position.x, position.y);
        let right_thruster_pos_2d = Vec2::new(position.x + 5.0, position.y);
        let left_thruster_pos_2d = Vec2::new(position.x - 5.0, position.y);
        let up = transform.up();
        let up2d = Vec2::new(up.x, up.y);

        //let mut external_force_inner = external_force.deref_mut();

        let mut external_force_new =ExternalForce::default();

        let thrust_power = 2.0;

        if ship.left_input {
            external_force_new += ExternalForce::at_point(up2d * thrust_power, right_thruster_pos_2d, position2d);
        }
        if ship.right_input {
            external_force_new += ExternalForce::at_point(up2d * thrust_power, left_thruster_pos_2d, position2d);
        }

        external_force.force = external_force_new.force;
        external_force.torque = external_force_new.torque;
    });
}

pub fn init_one_ship(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>){
    let cube = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });
    let cube_id = cube.id();

    commands
        .spawn_empty()
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        })
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.,
        })
        .insert(Collider::cuboid(30., 14.))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Ship::default())
        .insert(LocalPlayer)
    // add the cube as child
        .push_children(&[cube_id]);
}
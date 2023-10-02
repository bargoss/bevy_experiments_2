use bevy::prelude::{Assets, BuildChildren, Camera3dBundle, Color, Commands, default, Input, KeyCode, Mat4, Mesh, PbrBundle, Query, Res, ResMut, shape, StandardMaterial, Transform, TransformBundle, Vec2, Vec3, VisibilityBundle};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, ExternalForce, RapierConfiguration, RigidBody, Velocity};
use crate::games::ship_fight::{LocalPlayer, Ship};

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
        let torque_arm = 0.35;
        let right_thruster_pos_2d = Vec2::new(position.x + torque_arm, position.y);
        let left_thruster_pos_2d = Vec2::new(position.x - torque_arm, position.y);
        let up = transform.up();
        let up2d = Vec2::new(up.x, up.y);

        //let mut external_force_inner = external_force.deref_mut();

        let mut external_force_new =ExternalForce::default();

        let thrust_power = 100.0;

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

pub fn init_game(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut rapier_config: ResMut<RapierConfiguration>){

    rapier_config.gravity = Vec2::new(0.0, -7.0);

    // create camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // spawn a simple red cube at origin
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });

    create_simple_map(&mut commands, &mut materials, &mut meshes);
    create_player_ship(&mut commands, &mut materials, &mut meshes);
}
fn create_simple_map(commands: &mut Commands, materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &mut ResMut<Assets<Mesh>>) {
    // rapier2d cube bundle
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        ..default()
    })
        .insert(Transform::from_translation(Vec3::new(0.0, -10.0, 0.0)))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(0.5, 0.5));
}
fn create_player_ship(commands: &mut Commands, materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &mut ResMut<Assets<Mesh>>) {
    let cube = commands.spawn(PbrBundle {
        transform: Transform::default(),
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ..default()
    });
    let cube_id = cube.id();

    commands
        .spawn_empty()
        .insert(TransformBundle{
            local : Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::new(3.5, 2.0, 1.0),
                Default::default(),
                Vec3::new(0.0, 3.0, 0.0),
            )),
            ..default()
        })
        .insert(VisibilityBundle::default())
        //.insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.,
        })
        .insert(Velocity {
            linvel: Vec2::ZERO,
            angvel: 0.,
        })
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Ship::default())
        .insert(LocalPlayer)
        .push_children(&[cube_id]);
}
use bevy::{
    prelude::{default, AssetServer, Commands, Entity, Query, Res, Transform, Vec3, With},
    sprite::SpriteBundle,
    time::Time,
    window::{PrimaryWindow, Window},
};

use super::{components::Ground, GROUND_HEIGHT, GROUND_MOVE_SPEED, GROUND_WIDTH};

pub fn spawn_ground(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut spawned_ground_width = 0.0;
    while spawned_ground_width < window.width() {
        let x = spawned_ground_width + GROUND_WIDTH / 2.0;
        let y = GROUND_HEIGHT / 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/groundDirt.png"),
                ..default()
            },
            Ground {},
        ));

        spawned_ground_width += GROUND_WIDTH;
    }

    println!("Window width: {}", window.width());
    println!("Spawned ground width: {}", spawned_ground_width);
}

pub fn move_ground_left_over_time(
    mut commands: Commands,
    mut ground_query: Query<(Entity, &mut Transform), With<Ground>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let mut last_entity: Option<Entity> = None;

    for (entity, mut transform) in ground_query.iter_mut() {
        let direction = Vec3::new(-1.0, 0.0, 0.0);
        transform.translation += direction * GROUND_MOVE_SPEED * time.delta_seconds();

        if transform.translation.x + GROUND_WIDTH / 2.0 < 0.0 {
            commands.entity(entity).despawn();
        }

        last_entity = Some(entity);
    }

    if let Some(entity) = last_entity {
        let window = window_query.get_single().unwrap();
        let (_, transform) = ground_query.get(entity).unwrap();
        if transform.translation.x + GROUND_WIDTH / 2.0 <= window.width() {
            let x = window.width() + GROUND_WIDTH / 2.0;
            let y = GROUND_HEIGHT / 2.0;
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("sprites/groundDirt.png"),
                    ..default()
                },
                Ground {},
            ));
        }
    }
}

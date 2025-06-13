//! Handles player movement and rotation, making movement camera-relative.
use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use crate::components::types::{Player, Position};
use crate::systems::input::MovementInput;
use crate::systems::camera::MainCamera;

pub fn register(app: &mut App) {
    // Run movement in FixedUpdate for consistent physics
    app.add_systems(FixedUpdate, player_movement);
}

/// Moves the player based on input, relative to the camera's facing direction, and rotates the player to face movement.
fn player_movement(
    movement_input: Res<MovementInput>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<Player>)>,
    mut player_query: Query<(&Player, &mut Position, &mut Transform, &mut Velocity)>,
) {
    let (player, mut position, mut transform, mut velocity) = match player_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    let input = movement_input.0;
    if input == Vec2::ZERO {
        velocity.linvel = Vec3::ZERO;
        return;
    }

    let camera_transform = match camera_query.get_single() {
        Ok(t) => t,
        Err(_) => return,
    };

    // Cache camera vectors to avoid recalculating
    let forward = camera_transform.forward().xz().normalize_or_zero();
    let right = camera_transform.right().xz().normalize_or_zero();

    // Calculate movement direction in world space (single operation)
    let move_dir = (right * input.x + forward * input.y).normalize_or_zero();
    let move_vec = Vec3::new(move_dir.x, 0.0, move_dir.y) * player.speed;
    
    // Update velocity
    velocity.linvel = move_vec;

    // Only update rotation if we're moving
    if move_vec.length_squared() > 0.01 {
        let target_rot = Quat::from_rotation_y(move_vec.x.atan2(move_vec.z));
        // Use a fixed rotation speed for consistency
        transform.rotation = transform.rotation.slerp(target_rot, 0.2);
    }

    // Update position for gathering range checks
    position.value = transform.translation;
} 
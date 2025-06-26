//! Handles resource gathering logic and inventory tracking.
use bevy::prelude::*;
use bevy::prelude::Input;
use bevy::input::mouse::MouseButton;
use bevy::math::Vec3;
use bevy_rapier3d::prelude::*;
use crate::components::types::{Player, Position, Gatherable, ResourceNode, PlayerInventory, ResourceType};

pub fn register(app: &mut App) {
    // Run gathering in FixedUpdate for consistent timing
    app.add_systems(FixedUpdate, gather_resources)
        .add_systems(Update, handle_resource_click);
}

/// System for gathering resources when the player is close and facing the resource.
fn gather_resources(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut inventory: ResMut<PlayerInventory>,
    mut player_query: Query<(&Position, &mut Player, &Transform), With<Player>>,
    resource_query: Query<(Entity, &Position, &Gatherable, &Transform), With<ResourceNode>>,
) {
    let (player_position, mut player, player_transform) = match player_query.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    // Update gathering cooldown
    if !player.gathering_cooldown.finished() {
        player.gathering_cooldown.tick(time.delta());
        return;
    }

    // Only process gathering if E is pressed
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    // Cache player position and forward vector
    let player_pos = player_position.value;
    let player_forward = player_transform.forward().normalize_or_zero();
    let gathering_range_sq = player.gathering_range * player.gathering_range;

    // Find the closest valid resource
    let mut closest_resource = None;
    let mut closest_distance = f32::MAX;

    for (entity, resource_position, gatherable, resource_transform) in resource_query.iter() {
        let distance_sq = player_pos.distance_squared(resource_position.value);
        
        // Skip if too far
        if distance_sq > gathering_range_sq {
            continue;
        }

        // Check if player is facing the resource
        let to_resource = (resource_transform.translation - player_pos).normalize_or_zero();
        if player_forward.dot(to_resource) <= 0.7 {
            continue;
        }

        // Update closest resource if this one is closer
        if distance_sq < closest_distance {
            closest_distance = distance_sq;
            closest_resource = Some((entity, gatherable));
        }
    }

    // Process the closest valid resource
    if let Some((entity, gatherable)) = closest_resource {
        let current_amount = *inventory
            .resources
            .get(&gatherable.resource_type)
            .unwrap_or(&0);
        if current_amount < inventory.max_stack_size {
            // Add resource to inventory
            let new_amount = current_amount + 1;
            inventory
                .resources
                .insert(gatherable.resource_type, new_amount);
            
            // Despawn the resource node
            commands.entity(entity).despawn_recursive();
            
            // Reset gathering cooldown
            player.gathering_cooldown.reset();
            
            println!("Gathered {}! Total: {}", gatherable.resource_type.get_name(), new_amount);
        }
    }
}

fn handle_resource_click(
    mut commands: Commands,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<Input<MouseButton>>,
    player_query: Query<(&Transform, &Player)>,
    resource_query: Query<(Entity, &Transform, &Gatherable)>,
    mut inventory: ResMut<PlayerInventory>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera.single();
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if let Some(_ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            let (player_transform, player) = player_query.single();
            let player_pos = player_transform.translation;
            let gathering_range_squared = player.gathering_range * player.gathering_range;

            // Find the closest resource within range
            let mut closest_resource = None;
            let mut closest_distance = f32::MAX;

            for (entity, transform, gatherable) in resource_query.iter() {
                let distance_squared = transform.translation.distance_squared(player_pos);
                if distance_squared <= gathering_range_squared {
                    if distance_squared < closest_distance {
                        closest_distance = distance_squared;
                        closest_resource = Some((entity, gatherable));
                    }
                }
            }

            // If we found a resource within range, gather it
            if let Some((entity, gatherable)) = closest_resource {
                let current_amount = *inventory.resources.get(&gatherable.resource_type).unwrap_or(&0);
                if current_amount < inventory.max_stack_size {
                    inventory.resources.insert(gatherable.resource_type.clone(), current_amount + 1);
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

/// (Optional) Handles resource respawn if you want to add it back after a timer.
fn handle_resource_respawn(
    _time: Res<Time>,
    _resource_query: Query<(&mut Gatherable, &mut Visibility)>,
) {
    // No-op: resources are despawned when gathered. Implement respawn logic here if desired.
} 
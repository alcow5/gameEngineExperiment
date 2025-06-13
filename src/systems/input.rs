//! Handles keyboard input for player movement.
// Only processes input and sets a movement direction resource.
use bevy::prelude::*;

/// Resource to store the current movement direction from input
#[derive(Resource, Default)]
pub struct MovementInput(pub Vec2);

pub fn register(app: &mut App) {
    app.init_resource::<MovementInput>();
    // Run input system in FixedUpdate for consistent input handling
    app.add_systems(FixedUpdate, input_system);
}

/// System to read WASD input and update the movement direction resource
fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut movement_input: ResMut<MovementInput>,
) {
    // Use a single Vec2 for input to avoid multiple allocations
    let mut direction = Vec2::ZERO;
    
    // Combine all input checks into a single block
    if keyboard.any_pressed([KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD]) {
        if keyboard.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if keyboard.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
        if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }
        movement_input.0 = direction.normalize_or_zero();
    } else {
        movement_input.0 = Vec2::ZERO;
    }
} 
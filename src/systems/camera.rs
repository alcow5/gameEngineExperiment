//! Third-person camera system: follows and orbits the player, supports mouse-based rotation and zoom.
use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::components::types::Player;

#[derive(Component)]
pub struct MainCamera {
    pub distance: f32,
    pub angle: f32,
    pub height: f32,
}

pub fn register(app: &mut App) {
    // Run camera follow in FixedUpdate for smooth movement
    app.add_systems(FixedUpdate, camera_follow)
        // Keep camera control in Update for responsive input
        .add_systems(Update, camera_control);
}

/// Smoothly follows the player and orbits based on camera angle and distance.
fn camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
) {
    let (player_transform, (mut camera_transform, camera)) = match (player_query.get_single(), camera_query.get_single_mut()) {
        (Ok(p), Ok(c)) => (p, c),
        _ => return,
    };

    let target_pos = player_transform.translation;
    
    // Cache sin/cos calculations
    let angle_rad = camera.angle;
    let (sin, cos) = angle_rad.sin_cos();
    
    // Calculate desired position in a single operation
    let offset = Vec3::new(
        camera.distance * sin,
        camera.height,
        camera.distance * cos,
    );
    let desired_pos = target_pos + offset;

    // Use a fixed lerp speed for consistency
    let lerp_speed = 8.0 * time.delta_seconds();
    camera_transform.translation = camera_transform.translation.lerp(desired_pos, lerp_speed);
    camera_transform.look_at(target_pos, Vec3::Y);
}

/// Handles mouse input for camera rotation and zoom.
fn camera_control(
    mut camera_query: Query<&mut MainCamera>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut motion_evr: EventReader<bevy::input::mouse::MouseMotion>,
) {
    let mut camera = match camera_query.get_single_mut() {
        Ok(c) => c,
        Err(_) => return,
    };

    // Rotate camera with left or right mouse button
    if mouse.pressed(MouseButton::Right) || mouse.pressed(MouseButton::Left) {
        for ev in motion_evr.read() {
            camera.angle -= ev.delta.x * 0.01;
        }
    }

    // Process all scroll events at once
    let mut total_scroll = 0.0;
    for ev in scroll_evr.read() {
        total_scroll += ev.y;
    }
    if total_scroll != 0.0 {
        camera.distance = (camera.distance - total_scroll * 0.5).clamp(2.0, 10.0);
        camera.height = (camera.height - total_scroll * 0.2).clamp(1.0, 5.0);
    }
} 
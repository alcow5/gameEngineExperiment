//! Spawns and sets up all entities: player, trees, rocks, ground, camera, and light.
//! Also contains component registration for the world.
use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::components::types::{Player, Position, Gatherable, ResourceNode, GameAssets, ResourceType};
use crate::systems::camera::MainCamera;

pub fn register(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, spawn_resources);
}

/// Sets up the world: camera, light, ground, loads models, and spawns the player.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut game_assets: ResMut<GameAssets>,
) {
    // Spawn ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50.0, 0.5, 50.0),
    ));

    // Add a directional light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera {
            distance: 5.0,
            angle: 0.0,
            height: 2.5,
        },
    ));

    // Load models
    game_assets.player_model = asset_server.load("models/CharWalk.glb#Scene0");
    game_assets.tree_models = vec![
        asset_server.load("models/tree1.glb#Scene0"),
        asset_server.load("models/tree2.glb#Scene0"),
    ];
    game_assets.rock_model = asset_server.load("models/rock1.glb#Scene0");

    // Spawn a simple player cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.2, 0.2),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player {
            speed: 5.0,
            gathering_range: 2.0,
            gathering_cooldown: Timer::from_seconds(1.0, TimerMode::Once),
        },
        Position { value: Vec3::ZERO },
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Velocity::zero(),
        Friction::coefficient(0.7),
        Restitution::coefficient(0.3),
        Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        },
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
    ));

    // Spawn resources
    let mut rng = rand::thread_rng();
    
    // Spawn trees
    for _ in 0..20 {
        let x = rng.gen_range(-20.0..20.0);
        let z = rng.gen_range(-20.0..20.0);
        
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/tree1.glb#Scene0"),
                transform: Transform::from_xyz(x, 0.0, z),
                ..default()
            },
            Position { value: Vec3::new(x, 0.0, z) },
            ResourceNode,
            RigidBody::Fixed,
            Collider::cylinder(1.0, 0.5),
            Gatherable {
                resource_type: ResourceType::Wood,
                health: 100,
                respawn_timer: None,
            },
        ));
    }
    
    // Spawn rocks
    for _ in 0..10 {
        let x = rng.gen_range(-20.0..20.0);
        let z = rng.gen_range(-20.0..20.0);
        
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/rock1.glb#Scene0"),
                transform: Transform::from_xyz(x, 0.0, z),
                ..default()
            },
            Position { value: Vec3::new(x, 0.0, z) },
            ResourceNode,
            RigidBody::Fixed,
            Collider::cylinder(0.5, 0.5),
            Gatherable {
                resource_type: ResourceType::Stone,
                health: 100,
                respawn_timer: None,
            },
        ));
    }
}

/// Spawns resource nodes (trees and rocks) in a grid after assets are loaded.
fn spawn_resources(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    // Check if assets are loaded
    if game_assets.tree_models.is_empty() {
        return;
    }

    // Check if any assets are still loading
    for handle in &game_assets.tree_models {
        if asset_server.get_load_state(handle) != Some(LoadState::Loaded) {
            return;
        }
    }
    if asset_server.get_load_state(&game_assets.rock_model) != Some(LoadState::Loaded) {
        return;
    }

    // Keep resource spawning commented out
    /*
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let x = rng.gen_range(-20.0..20.0);
        let z = rng.gen_range(-20.0..20.0);
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/tree.glb#Scene0"),
                transform: Transform::from_xyz(x, 0.0, z),
                ..default()
            },
            Gatherable {
                resource_type: ResourceType::Wood,
                health: 100,
                respawn_timer: None,
            },
            RigidBody::Fixed,
            Collider::cylinder(1.0, 0.5),
        ));
    }

    for _ in 0..10 {
        let x = rng.gen_range(-20.0..20.0);
        let z = rng.gen_range(-20.0..20.0);
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/rock.glb#Scene0"),
                transform: Transform::from_xyz(x, 0.0, z),
                ..default()
            },
            Gatherable {
                resource_type: ResourceType::Stone,
                health: 100,
                respawn_timer: None,
            },
            RigidBody::Fixed,
            Collider::cylinder(0.5, 0.5),
        ));
    }
    */
} 
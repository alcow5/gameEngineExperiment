use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

// Components
#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Position {
    value: Vec3,
}

#[derive(Component)]
struct Gatherable {
    resource_type: ResourceType,
}

#[derive(Component)]
struct Tree;

#[derive(Component)]
struct WoodCountText;

// Resources
#[derive(Resource)]
struct PlayerInventory {
    wood: u32,
}

#[derive(Resource)]
struct GameAssets {
    player_color: Color,
    tree_color: Color,
}

// Enums
#[derive(Clone, Copy)]
enum ResourceType {
    Wood,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PlayerInventory { wood: 0 })
        .insert_resource(GameAssets {
            player_color: Color::rgb(0.25, 0.25, 0.75),
            tree_color: Color::rgb(0.0, 0.5, 0.0),
        })
        .add_systems(Startup, (setup, spawn_trees, setup_ui))
        .add_systems(Update, (
            player_movement,
            gather_resources,
            update_wood_count_text,
        ))
        .run();
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: game_assets.player_color,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Player { speed: 200.0 },
        Position { value: Vec3::ZERO },
    ));
}

fn spawn_trees(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn trees in a grid pattern
    for x in -2..=2 {
        for y in -2..=2 {
            if x == 0 && y == 0 { continue; } // Skip center where player spawns
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: game_assets.tree_color,
                        custom_size: Some(Vec2::new(40.0, 40.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 100.0,
                        y as f32 * 100.0,
                        0.0,
                    )),
                    ..default()
                },
                Tree,
                Position {
                    value: Vec3::new(x as f32 * 100.0, y as f32 * 100.0, 0.0),
                },
                Gatherable {
                    resource_type: ResourceType::Wood,
                },
            ));
        }
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Wood: 0",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        WoodCountText,
    ));
}

fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Position, &mut Transform)>,
) {
    let (player, mut position, mut transform) = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        position.value += direction * player.speed * time.delta_seconds();
        transform.translation = position.value;
    }
}

fn gather_resources(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut inventory: ResMut<PlayerInventory>,
    player_query: Query<&Position, With<Player>>,
    tree_query: Query<(Entity, &Position, &Gatherable), With<Tree>>,
) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        let player_position = player_query.single();
        
        for (entity, tree_position, gatherable) in tree_query.iter() {
            let distance = player_position.value.distance(tree_position.value);
            
            if distance < 60.0 {
                match gatherable.resource_type {
                    ResourceType::Wood => {
                        inventory.wood += 1;
                        commands.entity(entity).despawn();
                        println!("Gathered wood! Total: {}", inventory.wood);
                    }
                }
            }
        }
    }
}

fn update_wood_count_text(
    inventory: Res<PlayerInventory>,
    mut query: Query<&mut Text, With<WoodCountText>>,
) {
    if inventory.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Wood: {}", inventory.wood);
        }
    }
}

//! UI system for displaying the player's inventory (HUD).
use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use std::fs::OpenOptions;
use std::io::Write;
use crate::components::types::{InventoryText, PlayerInventory, ResourceType};

#[derive(Component)]
struct FpsText;

pub fn register(app: &mut App) {
    app.add_systems(Startup, setup_ui)
        // Update UI less frequently for better performance
        .add_systems(Update, update_inventory_text)
        .add_systems(Update, update_fps_text);
}

/// Sets up the inventory HUD in the top-left corner.
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Inventory text
    commands.spawn((
        TextBundle::from_section(
            "Inventory: Wood: 0, Stone: 0, Ore: 0",
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
        InventoryText,
    ));

    // FPS text
    commands.spawn((
        TextBundle::from_section(
            "FPS: 0",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(10.0),
            ..default()
        }),
        FpsText,
    ));
}

/// Updates the inventory HUD when the player's inventory changes.
fn update_inventory_text(
    inventory: Res<PlayerInventory>,
    mut query: Query<&mut Text, With<InventoryText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        let new_text = format!(
            "Inventory: Wood: {}, Stone: {}, Ore: {}",
            inventory.resources.get(&ResourceType::Wood).unwrap_or(&0),
            inventory.resources.get(&ResourceType::Stone).unwrap_or(&0),
            inventory.resources.get(&ResourceType::Ore).unwrap_or(&0)
        );
        if text.sections[0].value != new_text {
            text.sections[0].value = new_text;
        }
    }
}

pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[0].value = format!("FPS: {:.0}", value);
                // Log FPS to file
                if let Ok(mut file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("fps_log.txt") 
                {
                    let _ = writeln!(file, "{}", value);
                }
            }
        }
    }
} 
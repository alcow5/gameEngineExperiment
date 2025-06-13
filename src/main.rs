//! Entry point for the 3D resource gathering game using Bevy.
//! Sets up plugins, resources, and registers all systems.
mod components;
mod systems;

use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_rapier3d::prelude::*;
use std::collections::HashMap;

use components::types::{PlayerInventory, GameAssets};
use systems::{
    input::register as register_input,
    camera::register as register_camera,
    resources::register as register_resources,
    entities::register as register_entities,
    ui::register as register_ui,
    movement::register as register_movement,
};

fn main() {
    // Create the Bevy app
    let mut app = App::new();
    // Add core plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default()) // Disabled for performance
        // Add diagnostic plugins
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default());
    // Insert game resources
    app.insert_resource(PlayerInventory {
        resources: HashMap::new(),
        max_stack_size: 10,
    })
    .insert_resource(GameAssets {
        player_model: Handle::default(),
        tree_models: Vec::new(),
        rock_model: Handle::default(),
    });
    // Register all system modules
    register_input(&mut app);
    register_camera(&mut app);
    register_resources(&mut app);
    register_entities(&mut app);
    register_ui(&mut app);
    register_movement(&mut app);
    // Run the game
    app.run();
}

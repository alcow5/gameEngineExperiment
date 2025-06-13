//! Shared components and helper types for the resource gathering game.
use bevy::prelude::*;
use std::collections::HashMap;

/// Marker component for the player entity.
#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub gathering_range: f32,
    pub gathering_cooldown: Timer,
}

/// Stores the world position for entities.
#[derive(Component)]
pub struct Position {
    pub value: Vec3,
}

/// Component for gatherable resource nodes (trees, rocks).
#[derive(Component)]
pub struct Gatherable {
    pub resource_type: ResourceType,
    pub health: u32,
    pub respawn_timer: Option<Timer>,
}

/// Marker for resource node entities.
#[derive(Component)]
pub struct ResourceNode;

/// Marker for the inventory UI text entity.
#[derive(Component)]
pub struct InventoryText;

/// Resource: Player's inventory and stack size limit.
#[derive(Resource)]
pub struct PlayerInventory {
    pub resources: HashMap<ResourceType, u32>,
    pub max_stack_size: u32,
}

/// Resource: Handles to all loaded game assets.
#[derive(Resource)]
pub struct GameAssets {
    pub player_model: Handle<Scene>,
    pub tree_models: Vec<Handle<Scene>>,
    pub rock_model: Handle<Scene>,
}

/// Enum for all resource types in the game.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Wood,
    Stone,
    Ore,
}

impl ResourceType {
    /// Returns the display name for each resource type.
    pub fn get_name(&self) -> &'static str {
        match self {
            ResourceType::Wood => "Wood",
            ResourceType::Stone => "Stone",
            ResourceType::Ore => "Ore",
        }
    }
} 
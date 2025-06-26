use bevy_game_demo::components::types::ResourceType;

#[test]
fn wood_get_name() {
    assert_eq!(ResourceType::Wood.get_name(), "Wood");
}

#[test]
fn stone_get_name() {
    assert_eq!(ResourceType::Stone.get_name(), "Stone");
}

#[test]
fn ore_get_name() {
    assert_eq!(ResourceType::Ore.get_name(), "Ore");
}

use bevy::{color::palettes, prelude::*};

pub struct PlatformPlugin;
impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_platform);
    }
}

#[derive(Event)]
pub struct SpawnPlatform;

#[derive(Component)]
pub struct Platfrom;

fn spawn_platform(spawn: On<SpawnPlatform>, mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: palettes::basic::GRAY.into(),
            custom_size: Some(Vec2::splat(100.0)),
            ..Default::default()
        },
        Platfrom,
    ));
}

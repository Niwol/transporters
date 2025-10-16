use bevy::prelude::*;

use crate::{
    agent::{AgentPlugin, SpawnAgentEvent},
    drag_handle::DragHandlePlugin,
    platform::{PlatformPlugin, SpawnPlatform},
    plug::PlugPlugin,
    rail::{Rail, RailPlugin, SpawnRailEvent},
};

mod agent;
mod drag_handle;
mod platform;
mod plug;
mod rail;

pub struct TransporterGamePlugin;

impl Plugin for TransporterGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins((
            RailPlugin,
            AgentPlugin,
            PlatformPlugin,
            PlugPlugin,
            DragHandlePlugin,
        ));

        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            spawn_agent
                .run_if(|input: Res<ButtonInput<KeyCode>>| input.just_pressed(KeyCode::Space)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(ClearColor(Color::srgb(0.8, 0.8, 0.8)));

    commands.trigger(SpawnRailEvent {
        transform: Transform::from_xyz(-100.0, 150.0, 0.0),
        bezier: CubicBezier::new([[
            Vec2::new(-500.0, -300.0),
            Vec2::new(-100.0, 300.0),
            Vec2::new(100.0, -300.0),
            Vec2::new(500.0, 300.0),
        ]]),
    });

    commands.trigger(SpawnPlatform);
}

fn spawn_agent(mut commands: Commands, rail: Single<Entity, With<Rail>>) {
    commands.trigger(SpawnAgentEvent { rail: *rail });
}

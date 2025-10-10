use bevy::prelude::*;

use crate::{
    agent::{AgentPlugin, SpawnAgentEvent},
    rail::{Rail, RailPlugin, SpawnRailEvent},
};

mod agent;
mod rail;
pub struct TransporterGamePlugin;

impl Plugin for TransporterGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
        app.add_plugins((RailPlugin, AgentPlugin));

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

    commands.trigger(SpawnRailEvent {
        transform: Transform::from_xyz(-100.0, 150.0, 0.0),
        spline: CubicBSpline::new([
            Vec2::new(-500.0, -600.0),
            Vec2::new(-100.0, 300.0),
            Vec2::new(100.0, -300.0),
            Vec2::new(500.0, 300.0),
        ]),
    });
}

fn spawn_agent(mut commands: Commands, rail: Single<Entity, With<Rail>>) {
    commands.trigger(SpawnAgentEvent { rail: *rail });
}

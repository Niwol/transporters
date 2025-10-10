use bevy::{color::palettes, prelude::*};

use crate::rail::Rail;

pub struct AgentPlugin;
impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_agent);

        app.add_systems(Update, update_agents);
    }
}

#[derive(Event)]
pub struct SpawnAgentEvent {
    pub rail: Entity,
}

#[derive(Component)]
struct Agent {
    rail: Entity,
    progression: f32,
    direction: Direction,
}

enum Direction {
    Forward,
    Backward,
}

fn spawn_agent(spawn_agent: On<SpawnAgentEvent>, mut commands: Commands) {
    let agent = commands
        .spawn((
            Sprite {
                color: palettes::basic::BLUE.into(),
                custom_size: Some(Vec2::splat(16.0)),
                ..Default::default()
            },
            Agent {
                rail: spawn_agent.rail,
                progression: 0.0,
                direction: Direction::Forward,
            },
        ))
        .id();

    commands.entity(spawn_agent.rail).add_child(agent);
}

fn update_agents(
    time: Res<Time>,
    mut agents: Query<(&mut Agent, &mut Transform)>,
    rails: Query<&Rail>,
) {
    for (mut agent, mut transform) in &mut agents {
        let dt = time.delta_secs();
        match agent.direction {
            Direction::Forward => {
                agent.progression += dt;
                if agent.progression > 1.0 {
                    agent.progression = 1.0;
                    agent.direction = Direction::Backward;
                }
            }
            Direction::Backward => {
                agent.progression -= dt;
                if agent.progression < 0.0 {
                    agent.progression = 0.0;
                    agent.direction = Direction::Forward;
                }
            }
        }

        let rail = rails.get(agent.rail).unwrap();
        let sample = rail.curve.sample(agent.progression).unwrap();

        transform.translation = Vec3::new(sample.x, sample.y, 0.0);
    }
}

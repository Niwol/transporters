use bevy::{color::palettes, prelude::*};

const PLUG_SIZE: Vec2 = Vec2 { x: 16.0, y: 4.0 };

pub struct PlugPlugin;
impl Plugin for PlugPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(plug_bar_added);
    }
}

#[derive(Component)]
pub struct PlugBar {
    nb_plugs: usize,
}

impl PlugBar {
    pub fn new(nb_plugs: usize) -> Self {
        Self { nb_plugs }
    }
}

fn plug_bar_added(add: On<Add, PlugBar>, mut commands: Commands, plug_bars: Query<&PlugBar>) {
    let plug_bar = plug_bars.get(add.entity).unwrap();

    let plug_bar_size = Vec2 {
        x: (PLUG_SIZE.x + 8.0) * plug_bar.nb_plugs as f32 + 8.0,
        y: 8.0,
    };

    commands
        .entity(add.entity)
        .insert((
            Sprite {
                color: palettes::basic::BLACK.into(),
                custom_size: Some(plug_bar_size),

                ..Default::default()
            },
            Transform::from_xyz(200.0, 0.0, 0.0),
        ))
        .with_children(|parent| {
            for i in 0..plug_bar.nb_plugs {
                let start = -plug_bar_size.x / 2.0;
                let x = start + 8.0 + (PLUG_SIZE.x + 8.0) * i as f32 + PLUG_SIZE.x / 2.0;
                let y = 6.0;

                parent.spawn((
                    Sprite {
                        color: palettes::basic::YELLOW.into(),
                        custom_size: Some(PLUG_SIZE),
                        ..Default::default()
                    },
                    Transform::from_xyz(x, y, 0.0),
                ));
            }
        });
}

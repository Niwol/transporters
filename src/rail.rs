use std::fmt::Debug;

use bevy::{asset::RenderAssetUsages, color::palettes, mesh::PrimitiveTopology, prelude::*};

pub struct RailPlugin;

impl Plugin for RailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_rail_materials);

        app.add_observer(spawn_rail);
    }
}

#[derive(Resource)]
struct RailMaterials {
    basic: Handle<ColorMaterial>,
}

#[derive(Event)]
pub struct SpawnRailEvent {
    pub transform: Transform,
    pub spline: CubicBSpline<Vec2>,
}

#[derive(Component)]
pub struct Rail {
    pub curve: CubicCurve<Vec2>,
}

#[derive(Component)]
struct RailHandle;

fn load_rail_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let rail_materials = RailMaterials {
        basic: materials.add(ColorMaterial::from_color(palettes::basic::GRAY)),
    };

    commands.insert_resource(rail_materials);
}

fn spawn_rail(
    rail_spawn: On<SpawnRailEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    rail_materials: Res<RailMaterials>,
) {
    let mut mesh = Mesh::new(
        PrimitiveTopology::LineStrip,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    let curve = rail_spawn.spline.to_curve().unwrap();
    let vertices = curve
        .iter_positions(100)
        .map(|v| Vec3::new(v.x, v.y, 0.0))
        .collect::<Vec<_>>();

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices, // vec![[-100.0, 0.0, 0.0], [0.0, 100.0, 0.0], [100.0, 0.0, 0.0]],
    );

    let handle = meshes.add(mesh);

    commands
        .spawn((
            Rail { curve },
            rail_spawn.transform,
            Mesh2d(handle),
            MeshMaterial2d(rail_materials.basic.clone()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Sprite {
                        color: palettes::basic::GREEN.into(),
                        custom_size: Some(Vec2::splat(16.0)),
                        ..Default::default()
                    },
                    RailHandle,
                    Pickable::default(),
                ))
                .observe(color_change::<Pointer<Press>>(Color::srgb(0.0, 1.0, 0.0)))
                .observe(color_change::<Pointer<Over>>(Color::srgb(0.0, 0.7, 0.0)))
                .observe(color_change::<Pointer<Out>>(Color::srgb(0.0, 0.5, 0.0)))
                .observe(drag_rail);
        });
}

fn color_change<E: EntityEvent + Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(On<E>, Query<&mut Sprite>) {
    move |event, mut sprites| {
        if let Ok(mut sprite) = sprites.get_mut(event.event_target()) {
            sprite.color = color;
        }
    }
}

fn drag_rail(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    children: Query<&ChildOf>,
) {
    if let Ok(child) = children.get(drag.entity) {
        let mut transform = transforms.get_mut(child.parent()).unwrap();
        transform.translation += Vec3::new(drag.delta.x, -drag.delta.y, 0.0);
    }
}

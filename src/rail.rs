use std::fmt::Debug;

use bevy::{asset::RenderAssetUsages, color::palettes, mesh::PrimitiveTopology, prelude::*};

use crate::drag_handle::DragHandle;

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
    pub bezier: CubicBezier<Vec2>,
}

#[derive(Component)]
pub struct Rail {
    pub curve: CubicCurve<Vec2>,
    pub bezier: CubicBezier<Vec2>,
}

#[derive(Component)]
struct ControlPoint {
    index: usize,
}

fn load_rail_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let rail_materials = RailMaterials {
        basic: materials.add(ColorMaterial::from_color(palettes::basic::BLACK)),
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

    let curve = rail_spawn.bezier.to_curve().unwrap();
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
            Rail {
                curve,
                bezier: rail_spawn.bezier.clone(),
            },
            rail_spawn.transform,
            Mesh2d(handle),
            MeshMaterial2d(rail_materials.basic.clone()),
            DragHandle::default(),
        ))
        .with_children(|parent| {
            for (index, control_point) in rail_spawn.bezier.control_points[0].iter().enumerate() {
                parent
                    .spawn((
                        Sprite {
                            color: palettes::basic::PURPLE.into(),
                            custom_size: Some(Vec2::splat(16.0)),
                            ..Default::default()
                        },
                        Transform::from_xyz(control_point.x, control_point.y, 0.0),
                        ControlPoint { index },
                        Pickable::default(),
                    ))
                    .observe(color_change::<Pointer<Press>>(Color::srgb(1.0, 0.0, 1.0)))
                    .observe(color_change::<Pointer<Over>>(Color::srgb(0.7, 0.0, 0.7)))
                    .observe(color_change::<Pointer<Out>>(Color::srgb(0.5, 0.0, 0.5)))
                    .observe(color_change::<Pointer<Release>>(Color::srgb(0.5, 0.0, 0.5)))
                    .observe(drag_controll_point);
            }
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

fn drag_controll_point(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<(&mut Transform, &ControlPoint)>,
    children: Query<&ChildOf>,
    mut rails: Query<(&mut Rail, &Mesh2d)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if let Ok((mut transform, control_point)) = transforms.get_mut(drag.entity) {
        transform.translation += Vec3::new(drag.delta.x, -drag.delta.y, 0.0);
        let new_pos = transform.translation.xy();

        let (mut rail, mesh_2d) = rails
            .get_mut(children.get(drag.entity).unwrap().parent())
            .unwrap();

        rail.bezier.control_points[0][control_point.index] = new_pos;
        rail.curve = rail.bezier.to_curve().unwrap();

        let mesh = meshes.get_mut(mesh_2d.0.id()).unwrap();

        let vertices = rail
            .curve
            .iter_positions(100)
            .map(|v| Vec3::new(v.x, v.y, 0.0))
            .collect::<Vec<_>>();

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vertices, // vec![[-100.0, 0.0, 0.0], [0.0, 100.0, 0.0], [100.0, 0.0, 0.0]],
        );
    }
}

use std::{fmt::Debug, panic::UnwindSafe};

use avian2d::prelude::{Collider, RigidBody, Sensor};
use bevy::{asset::RenderAssetUsages, color::palettes, mesh::PrimitiveTopology, prelude::*};

use crate::{
    drag_handle::{DragHandle, DragStop},
    plug::Plug,
};

pub struct RailPlugin;

impl Plugin for RailPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_rail_materials);

        app.add_systems(Update, update_rail_curve);

        app.add_observer(spawn_rail);
    }
}

#[derive(Resource)]
struct RailMaterials {
    basic: Handle<ColorMaterial>,
}

#[derive(Event)]
pub struct SpawnRailEvent {
    pub _transform: Transform,
    pub bezier: CubicBezier<Vec2>,
}

#[derive(Component)]
pub struct Rail {
    pub curve: CubicCurve<Vec2>,
    pub bezier: CubicBezier<Vec2>,
}

#[derive(Component)]
struct ControlPoint {
    rail: Entity,
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

    let mesh_handle = meshes.add(mesh);

    let rail = commands
        .spawn((
            Rail {
                curve,
                bezier: rail_spawn.bezier.clone(),
            },
            Mesh2d(mesh_handle),
            MeshMaterial2d(rail_materials.basic.clone()),
            DragHandle::default(),
        ))
        .id();

    spawn_control_point(
        &mut commands,
        rail,
        0,
        rail_spawn.bezier.control_points[0][0],
    );
    spawn_control_point(
        &mut commands,
        rail,
        1,
        rail_spawn.bezier.control_points[0][1],
    );
    spawn_control_point(
        &mut commands,
        rail,
        2,
        rail_spawn.bezier.control_points[0][2],
    );
    spawn_control_point(
        &mut commands,
        rail,
        3,
        rail_spawn.bezier.control_points[0][3],
    );

    // .with_children(|parent| {
    //     for (index, control_point) in rail_spawn.bezier.control_points[0].iter().enumerate() {
    //         parent.spawn((
    //             // Sprite {
    //             //     color: palettes::basic::PURPLE.into(),
    //             //     custom_size: Some(Vec2::splat(16.0)),
    //             //     ..Default::default()
    //             // },
    //             Transform::from_xyz(control_point.x, control_point.y, 0.0),
    //             ControlPoint { index },
    //             Pickable::default(),
    //             RigidBody::Static,
    //             Collider::rectangle(5.0, 5.0),
    //             Sensor,
    //             DragHandle::default(),
    //         ));
    // .observe(color_change::<Pointer<Press>>(Color::srgb(1.0, 0.0, 1.0)))
    // .observe(color_change::<Pointer<Over>>(Color::srgb(0.7, 0.0, 0.7)))
    // .observe(color_change::<Pointer<Out>>(Color::srgb(0.5, 0.0, 0.5)))
    // .observe(color_change::<Pointer<Release>>(Color::srgb(0.5, 0.0, 0.5)))
    // .observe(drag_controll_point);
    // }
    // });
}

fn spawn_control_point(
    commands: &mut Commands,
    rail: Entity,
    index: usize,
    position: Vec2,
) -> Entity {
    let mut entity_commands = commands.spawn((
        ControlPoint { rail, index },
        Transform::from_xyz(position.x, position.y, 0.0),
        DragHandle::default(),
    ));

    if index == 0 || index == 3 {
        entity_commands.observe(on_release);
    }

    entity_commands.id()
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
    control_points: Query<(&Transform, &ChildOf, &ControlPoint), Changed<Transform>>,
) {
    for (transform, child_of, control_point) in control_points {}

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

fn update_rail_curve(
    control_points: Query<(&Transform, &ControlPoint), Changed<Transform>>,
    mut rails: Query<(&mut Rail, &Mesh2d)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (transform, control_point) in &control_points {
        let (mut rail, mesh) = rails.get_mut(control_point.rail).unwrap();

        rail.bezier.control_points[0][control_point.index] = transform.translation.xy();
        rail.curve = rail.bezier.to_curve().unwrap();

        let mesh = meshes.get_mut(mesh.0.id()).unwrap();

        let vertices = rail
            .curve
            .iter_positions(100)
            .map(|v| Vec3::new(v.x, v.y, 0.0))
            .collect::<Vec<_>>();

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    }
}

fn on_release(release: On<DragStop>, plugs: Query<&Transform, With<Plug>>) {
    println!("Released");
}

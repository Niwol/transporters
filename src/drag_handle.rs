use bevy::{color::palettes, prelude::*};

pub struct DragHandlePlugin;
impl Plugin for DragHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(added_drag_handle);
    }
}

#[derive(Component)]
pub struct DragHandle {
    pub offset: Vec2,
}

impl Default for DragHandle {
    fn default() -> Self {
        Self { offset: Vec2::ZERO }
    }
}

#[derive(Component)]
struct InnerDragHandle;

fn added_drag_handle(
    add: On<Add, DragHandle>,
    mut commands: Commands,
    drag_handles: Query<&DragHandle>,
) {
    let handle = drag_handles.get(add.entity).unwrap();

    let inner_handle = commands
        .spawn((
            Sprite {
                color: palettes::basic::GREEN.into(),
                custom_size: Some(Vec2::splat(16.0)),
                ..Default::default()
            },
            Transform::from_xyz(handle.offset.x, handle.offset.y, 0.0),
            InnerDragHandle,
            Pickable::default(),
        ))
        .observe(color_change::<Pointer<Press>>(Color::srgb(0.0, 1.0, 0.0)))
        .observe(color_change::<Pointer<Over>>(Color::srgb(0.0, 0.7, 0.0)))
        .observe(color_change::<Pointer<Out>>(Color::srgb(0.0, 0.5, 0.0)))
        .observe(color_change::<Pointer<Release>>(Color::srgb(0.0, 0.5, 0.0)))
        .observe(drag_parent)
        .id();

    commands.entity(add.entity).add_child(inner_handle);
}

fn color_change<E: EntityEvent + Clone + Reflect>(
    color: Color,
) -> impl Fn(On<E>, Query<&mut Sprite>) {
    move |event, mut sprites| {
        if let Ok(mut sprite) = sprites.get_mut(event.event_target()) {
            sprite.color = color;
        }
    }
}

fn drag_parent(
    drag: On<Pointer<Drag>>,
    mut transforms: Query<&mut Transform>,
    children: Query<&ChildOf>,
) {
    if let Ok(child) = children.get(drag.entity) {
        let mut transform = transforms.get_mut(child.parent()).unwrap();
        transform.translation += Vec3::new(drag.delta.x, -drag.delta.y, 0.0);
    }
}

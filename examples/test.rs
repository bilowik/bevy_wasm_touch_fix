use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy_wasm_touch_fix::*;

pub fn main() {
    App::new()
        .add_plugins(WasmTouchFixPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: false,
                canvas: Some(String::from("#main-canvas")),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, init)
        .add_systems(Update, (handle_button, handle_touch_input))
        .run();
}

pub fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(128.),
                        height: Val::Px(128.),
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_items: JustifyItems::Center,
                        justify_self: JustifySelf::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::RED.into(),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn(
                        TextBundle::from_section(
                            "UI Button",
                            TextStyle {
                                font_size: 32.,
                                ..default()
                            },
                        )
                        .with_text_alignment(TextAlignment::Center),
                    );
                });
        });
}

pub fn handle_button(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut bg) in buttons.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                info!("Button has been pressed!");
                *bg = Color::GREEN.into();
            }
            Interaction::None => {
                *bg = Color::RED.into();
            }
            _ => {}
        }
    }
}

pub fn handle_touch_input(
    mut touch_input_events: EventReader<TouchInput>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();
    for e in touch_input_events.read() {
        info!("Screen-relative Pos {:?}", e.position);

        match e.phase {
            TouchPhase::Started => {
                // Transform the position using the camera's viewport into the world
                // since it is currently relative to the screen.
                // If somehow it fails default ot the original position.
                let position = camera
                    .viewport_to_world(camera_transform, e.position)
                    .map(|ray| ray.origin.truncate())
                    .unwrap_or(e.position);
                info!("In-World Pos {:?}", position);
                commands.spawn(ColorMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(4.0).into()).into(),
                    transform: Transform::from_translation(position.extend(0.0)),
                    material: color_materials.add(ColorMaterial::from(Color::GREEN)),
                    ..default()
                });
            }
            _ => {}
        }
    }
}

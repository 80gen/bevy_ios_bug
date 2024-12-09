use bevy::{log::LogPlugin, prelude::*};

#[cfg(any(target_os = "android", target_os = "ios"))]
fn make_window() -> Window {
    use bevy::window::WindowMode;

    Window {
        resizable: false,
        mode: WindowMode::BorderlessFullscreen,
        recognize_rotation_gesture: true,
        ..default()
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn make_window() -> Window {
    use bevy::window::EnabledButtons;

    Window {
        resizable: false,
        enabled_buttons: EnabledButtons {
            minimize: false,
            maximize: false,
            ..Default::default()
        },
        ..default()
    }
}

pub fn run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: make_window().into(),
                    ..default()
                })
                .set(LogPlugin {
                    custom_layer: |_| None,
                    filter: "info,winit=error".into(),
                    level: bevy::log::Level::INFO,
                }),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    let init = Transform::from_translation(Vec3::new(0., 0., 20.)).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(Camera3dBundle {
        camera: Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        camera_3d: Camera3d::default(),
        transform: init,
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

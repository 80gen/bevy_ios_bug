use bevy::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
fn make_window() -> Window {
    Window {
        resizable: false,
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: make_window().into(),
            ..default()
        }))
        .run();
}

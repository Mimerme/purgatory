use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::{info, AssetEvent, EventReader, Shader},
    window::CursorMoved,
};

pub fn print_mouse_events(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }
}

pub fn print_asset_events(mut reader: EventReader<AssetEvent<Shader>>) {
    for event in reader.iter() {
        match event {
            AssetEvent::Created { handle } => {
                // asset just finished loading
                info!("Shader asset loaded: {:?}", handle);
            }
            AssetEvent::Modified { handle: _ } => {
                // asset was changed
                info!("Shader asset reloaded");
            }
            AssetEvent::Removed { handle: _ } => {
                // asset was unloaded
                info!("Shader asset unloaded");
            }
        }
    }
}

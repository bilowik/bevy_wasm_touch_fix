#![doc = include_str!("../README.md")]
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::{touch::touch_screen_input_system, touch::TouchInput};
use bevy_log::*;
use bevy_math::prelude::*;

/// Additional offset to be added to touch events that can be set in case
/// web_sys::Element::get_bounding_client_rect alone does not provide an accurate
/// offset.
#[derive(Resource, Default)]
pub struct AdditionalTouchOffset(pub Vec2);

#[derive(Resource, Default)]
pub struct TouchOffset(pub Vec2);

/// Contains the Id of the primary canvas of the app. This needs to match the id selector
/// of the canvas or the offset cannot be calculated. Defaults to "main-canvas"
///
/// For example:
/// ```html
/// <canvas id="main-canvas"></canvas>
/// ```
/// in the html where you define the page that will be rendering the Bevy app.
///
/// Then you need to configure the Window plugin to render to that ID as well:
/// ```rust
/// let mut window_plugin = WindowPlugin::default();
/// window_plugin.primary_window = Some(String::from("#main-canvas"));
///
/// App::new()
///     .with_plugins(DefaultPlugins.set(window_plugin))
/// ```
///
#[derive(Resource)]
pub struct PrimaryCanvasId(pub String);

impl Default for PrimaryCanvasId {
    fn default() -> Self {
        Self("main-canvas".to_string())
    }
}

/// Event that is fired whenever the offset is changed.
#[derive(Event)]
pub struct TouchOffsetChangeEvent(pub Vec2);

/// Fixes the issue with touch inputs in wasm by automatically calculating and apply the offset of
/// the canvas element and applies it to all touch inputs before they are processed. `Touches`
/// will also reflect this offset and be accurate.
///
/// See [PrimaryCanvasId] on how to ensure you are targetting the correct canvas.
#[derive(Default)]
pub struct WasmTouchFixPlugin;

impl Plugin for WasmTouchFixPlugin {
    fn build(&self, app: &mut App) {
        app
            // We want to run it before the touch_screen_input_system so that it properly
            // propogates to the Touches resource.
            .add_systems(
                PreUpdate,
                (check_canvas_offset, offset_touch_input_events)
                    .chain()
                    .before(touch_screen_input_system),
            )
            .add_event::<TouchOffsetChangeEvent>()
            .init_resource::<TouchOffset>()
            .init_resource::<AdditionalTouchOffset>()
            .init_resource::<PrimaryCanvasId>();
    }
}

/// Checks and updates the touch offset if the offset of the canvas has changed.
pub fn check_canvas_offset(
    primary_canvas_id: Res<PrimaryCanvasId>,
    mut touch_offset: ResMut<TouchOffset>,
    mut touch_offset_change_event_writer: EventWriter<TouchOffsetChangeEvent>,
    mut error_count: Local<u32>,
) {
    if let Some(curr_offset) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(&primary_canvas_id.0))
        .map(|element| element.get_bounding_client_rect())
        .map(|rect| Vec2::new(rect.left() as f32, rect.top() as f32))
    {
        if curr_offset != touch_offset.0 {
            touch_offset.0 = curr_offset;
            touch_offset_change_event_writer.send(TouchOffsetChangeEvent(curr_offset));
            info!("New touch offset calculated: {:?}", curr_offset);
        }
    } else {
        *error_count += 1;
        if *error_count < 10 {
            error!("Failed to get the canvas offset using element id '{}', touch inputs may be impacted", primary_canvas_id.0);
        } else if *error_count == 10 {
            error!("Continued failure to get the canvas offset, silencing future error of this type. Make sure you properly set the canvas ID and that it matches the one set in the resource");
        }
    }
}

/// Applies the offset to all current touch input events
pub fn offset_touch_input_events(
    mut touch_input_events: ResMut<Events<TouchInput>>,
    touch_offset: Res<TouchOffset>,
    additional_touch_offset: Res<AdditionalTouchOffset>,
) {
    if !touch_input_events.is_empty() {
        debug!("Touch Events Pre-Transform: {:?}", touch_input_events);

        // Drain the older events and offset them.
        let old_events = touch_input_events
            .update_drain()
            .map(|mut e| {
                e.position -= touch_offset.0 + additional_touch_offset.0;
                e
            })
            .collect::<Vec<_>>();

        // Drain the newer events now and offset them.
        let new_events = touch_input_events
            .update_drain()
            .map(|mut e| {
                e.position -= touch_offset.0 + additional_touch_offset.0;
                e
            })
            .collect::<Vec<_>>();

        // Both event buffers should be empty now.

        // Now push the old ones back on
        for old_event in old_events.into_iter() {
            touch_input_events.send(old_event);
        }

        // Call update to move the old_events into the second buffer where they originally were
        touch_input_events.update();

        // Now push the new events back on
        for transformed_event in new_events.into_iter() {
            touch_input_events.send(transformed_event);
        }

        debug!("Touch Events Post-Transform: {:?}", touch_input_events);
    }
}

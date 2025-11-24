use std::{
    collections::HashMap,
    f32::consts::{FRAC_PI_2, PI},
    ops::Range,
};

use bevy::{
    camera::ScalingMode,
    ecs::system::SystemId,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};
use serde::{Deserialize, Serialize};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, load_menu, load_level).chain())
        .add_systems(Update, (orbit, switch_projection, zoom))
        .run();
}

#[derive(Debug, Resource)]
struct CameraSettings {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    pub orthographic_viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,
    /// Clamp perspective camera's field of view to this range
    pub perspective_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the perspective camera
    pub perspective_zoom_speed: f32,
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orthographic_viewport_height: 5.,
            // In orthographic projections, we specify camera scale relative to a default value of 1,
            // in which one unit in world space corresponds to one pixel.
            orthographic_zoom_range: 1f32..2.0,
            // This value was hand-tuned to ensure that zooming in and out feels smooth but not slow.
            orthographic_zoom_speed: 0.2,
            // Perspective projections use field of view, expressed in radians. We would
            // normally not set it to more than π, which represents a 180° FOV.
            perspective_zoom_range: (PI / 12f32)..(PI / 5f32),
            // Changes in FOV are much more noticeable due to its limited range in radians
            perspective_zoom_speed: 0.05,
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
        }
    }
}

#[derive(Resource)]
struct GameData {
    load_menu_id: SystemId,
    load_level_id: SystemId,
    load_editor_id: SystemId,
    load_custom_level_id: SystemId,
}

#[derive(Serialize, Deserialize, Asset, TypePath)]
struct LevelData {
    data: HashMap<IVec3, ObjectType>,
}

#[derive(Serialize, Deserialize)]
enum ObjectType {
    Cube,
    Ramp(Quat),
}

#[derive(Component)]
struct IsProjectionOrtho(bool);

fn load_menu(mut commands: Commands) {
    /*
    commands.spawn((
        Name::new("Instructions"),
        Text::new(
            "Mouse up or down: pitch\n\
            Mouse left or right: yaw\n\
            Mouse buttons: roll",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
    */
}

fn load_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("CurrentProjection"),
        Text::new("Orthographic"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
        IsProjectionOrtho(true),
    ));

    let camera_settings = CameraSettings::default();

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            // We can set the scaling mode to FixedVertical to keep the viewport height constant as its aspect ratio changes.
            // The viewport height is the height of the camera's view in world units when the scale is 1.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_settings.orthographic_viewport_height,
            },
            // This is the default value for scale for orthographic projections.
            // To zoom in and out, change this value, rather than `ScalingMode` or the camera's position.
            scale: 1.,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(camera_settings);

    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
    ));

    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.5, 0.51, 1.5),
    ));

    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));
}
fn load_editor(mut commands: Commands) {}
fn load_custom_level(mut commands: Commands) {}

/// Set up a simple 3D scene
fn setup(mut commands: Commands) {
    let game_data = GameData {
        load_menu_id: commands.register_system(load_menu),
        load_level_id: commands.register_system(load_level),
        load_editor_id: commands.register_system(load_editor),
        load_custom_level_id: commands.register_system(load_custom_level),
    };

    commands.insert_resource(game_data);
}

fn switch_projection(
    mut camera: Single<&mut Projection, With<Camera>>,
    current_projection: Single<(&mut IsProjectionOrtho, &mut Text)>,
    camera_settings: Res<CameraSettings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (mut is_ortho, mut text_projection) = current_projection.into_inner();
        if is_ortho.0 {
            *text_projection = "Perspective".into();
        } else {
            *text_projection = "Orthographic".into();
        }

        (*is_ortho).0 = !(*is_ortho).0;
        // Switch projection type
        **camera = match **camera {
            Projection::Orthographic(_) => Projection::Perspective(PerspectiveProjection {
                fov: camera_settings.perspective_zoom_range.start,
                ..default()
            }),
            Projection::Perspective(_) => Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: camera_settings.orthographic_viewport_height,
                },
                ..OrthographicProjection::default_3d()
            }),
            _ => return,
        }
    }
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
                camera_settings.orthographic_zoom_range.start,
                camera_settings.orthographic_zoom_range.end,
            );
        }
        Projection::Perspective(ref mut perspective) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.perspective_zoom_speed;

            // Adjust the field of view, but keep it within our stated range.
            perspective.fov = (perspective.fov + delta_zoom).clamp(
                camera_settings.perspective_zoom_range.start,
                camera_settings.perspective_zoom_range.end,
            );
        }
        _ => (),
    }
}

fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let target = Vec3::ZERO;
    let delta = mouse_motion.delta;
    let mut delta_roll = 0.0;

    // Mouse motion is one of the few inputs that should not be multiplied by delta time,
    // as we are already receiving the full movement since the last frame was rendered. Multiplying
    // by delta time here would make the movement slower that it should be.
    let delta_pitch = -(delta.y * camera_settings.pitch_speed);
    let delta_yaw = -(delta.x * camera_settings.yaw_speed);

    // Conversely, we DO need to factor in delta time for mouse button inputs.
    delta_roll *= camera_settings.roll_speed * time.delta_secs();

    // Obtain the existing pitch, yaw, and roll values from the transform.
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
    );
    let roll = roll + delta_roll;
    let yaw = yaw + delta_yaw;

    // Only orbit if pressing left mouse button
    if mouse_buttons.pressed(MouseButton::Right) {
        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    };

    // Adjust the translation to maintain the correct orientation toward the orbit target.
    // In our example it's a static target, but this could easily be customized.
    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}

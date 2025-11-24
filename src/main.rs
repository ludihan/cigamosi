use std::{
    f32::consts::{FRAC_PI_2, PI},
    ops::Range,
};

use bevy::{
    camera::ScalingMode,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};

mod editor;
mod game;
mod menu;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Editor,
    Level,
    Settings,
    CustomLevel,
    Exit,
}

#[derive(Resource, Debug)]
struct LevelIndex(usize);

#[derive(Resource, Debug)]
struct CustomLevel(std::path::PathBuf);

#[derive(Resource, Debug)]
struct Levels(Vec<Level>);

#[derive(Debug)]
struct Level {}

fn main() {
    App::new()
        .add_plugins(game::GamePlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct IsProjectionOrtho(bool);

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

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
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
}

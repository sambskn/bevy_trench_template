use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

// define some classes for trenchbroom to pick up
#[point_class(
    model({ path: "models/player.png", scale: 0.5 }),
)]
#[derive(Default)]
struct PlayerStart;

#[point_class]
#[derive(Default)]
struct Enemy {
    pub health: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FreeCameraPlugin)
        .add_plugins(
            TrenchBroomPlugins(
                TrenchBroomConfig::new("bevy_trench_test")
                    .default_solid_scene_hooks(|| SceneHooks::new().smooth_by_default_angle()),
            )
            .build(),
        )
        .add_plugins((CameraPlugin, TrenchLoaderPlugin))
        .run();
}

// Plugin that spawns the camera.
struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        // This component stores all camera settings and state, which is used by the FreeCameraPlugin to
        // control it. These properties can be changed at runtime, but beware the controller system is
        // constantly using and modifying those values unless the enabled field is false.
        FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 3.0,
            run_speed: 9.0,
            ..default()
        },
    ));
}

// Plugin that loads trenchbroom map
struct TrenchLoaderPlugin;
impl Plugin for TrenchLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_map);
    }
}

fn spawn_test_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneRoot(asset_server.load("maps/test.map#Scene")));
}

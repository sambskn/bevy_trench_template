use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

// point_class marks it for bevy_trenchbroom
// - adding a model path is for display in trenchbroom, not pulled for bevy side atm
// component is the bevy macro used to set up the hook for spawning our billboarded sprite
// (calling the on_add fn below)
#[point_class(
    model({ path: "sprites/rat.png", scale: 0.5 }),
)]
#[component(on_add = Self::on_add)]
#[derive(Default)]
struct NPCSprite;
impl NPCSprite {
    pub fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };
        let rect_mesh = asset_server.add(Mesh::from(Rectangle::new(0.42, 0.42)));
        let material = asset_server.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("sprites/rat.png")),
            perceptual_roughness: 1.0,
            alpha_mode: AlphaMode::Mask(1.0),
            cull_mode: None,
            ..default()
        });
        world
            .commands()
            .entity(ctx.entity)
            .insert((Mesh3d(rect_mesh), MeshMaterial3d(material)));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FreeCameraPlugin)
        .add_plugins(
            TrenchBroomPlugins(
                TrenchBroomConfig::new("bevy_trench_template")
                    .default_solid_scene_hooks(|| SceneHooks::new().smooth_by_default_angle()),
            )
            .build(),
        )
        .add_plugins((CameraPlugin, TrenchLoaderPlugin, BillboardSpritePlugin))
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
    // currently a debug camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

// Plugin for keeping billboard sprites facing the camera
struct BillboardSpritePlugin;
impl Plugin for BillboardSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_billboards);
    }
}

fn update_billboards(
    camera_query: Query<&Transform, (With<Camera3d>, Without<NPCSprite>)>,
    mut sprite_query: Query<&mut Transform, (With<NPCSprite>, Without<Camera3d>)>,
) {
    let Ok(cam_tf) = camera_query.single() else {
        return;
    };
    for mut sprite_tf in &mut sprite_query {
        sprite_tf.look_to(cam_tf.translation, Vec3::Y);
    }
}

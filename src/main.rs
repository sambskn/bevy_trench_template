use avian3d::{math::*, prelude::*};
use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    input::{common_conditions::input_just_pressed, mouse::AccumulatedMouseMotion},
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_trenchbroom::prelude::*;
use bevy_trenchbroom_avian::AvianPhysicsBackend;

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

const GRAVITY_MULT: f32 = 160.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // show pixels
                .set(WindowPlugin {
                    // setup window
                    primary_window: Window {
                        fit_canvas_to_parent: true, // make it fill on web
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        )
        .add_plugins((
            PhysicsPlugins::default(),
            TrenchBroomPhysicsPlugin::new(AvianPhysicsBackend),
        ))
        .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY_MULT))
        .add_plugins(
            TrenchBroomPlugins(
                TrenchBroomConfig::new("bevy_trench_template").default_solid_scene_hooks(|| {
                    SceneHooks::new()
                        .smooth_by_default_angle()
                        .convex_collider()
                }),
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
        app.add_systems(Startup, spawn_camera)
            .add_systems(FixedUpdate, (player_camera_movement, debug_comamnds))
            .add_systems(
                Update,
                (
                    update_camera_transform,
                    capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
                    release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
                ),
            );

        // Print info message with control info to console
        info!("\n\n\nControls:\n\tWASD => move, Mouse => look, Space => 'jump'\n\n");
    }
}

#[derive(Component)]
struct PlayerCamera;

const PLAYER_START_LOC: Transform = Transform::from_xyz(0., 10., 5.);

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        PlayerCamera,
        Camera3d::default(),
        PLAYER_START_LOC.clone(),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        TransformInterpolation,
        CollidingEntities::default(),
        LockedAxes::ROTATION_LOCKED,
    ));
}

const PLAYER_SPEED: f32 = 7.0;
const PLAYER_SPRINT_BOOST: f32 = 3.0;
const PLAYER_SLOWDOWN_MULT: f32 = 20.0;

fn player_camera_movement(
    mut query: Query<(&mut LinearVelocity, &Transform), With<PlayerCamera>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut lin_vel, camera) in &mut query {
        // build movement vec from current inputs
        let mut movement_vel = Vec3::ZERO;
        if input.pressed(KeyCode::KeyW) {
            movement_vel += Vec3::NEG_Z
        }
        if input.pressed(KeyCode::KeyS) {
            movement_vel += Vec3::Z
        }
        if input.pressed(KeyCode::KeyA) {
            movement_vel += Vec3::NEG_X
        }
        if input.pressed(KeyCode::KeyD) {
            movement_vel += Vec3::X
        }
        if input.pressed(KeyCode::Space) || input.pressed(KeyCode::KeyE) {
            movement_vel += Vec3::Y
        }
        if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::KeyQ) {
            movement_vel += Vec3::NEG_Y
        }
        movement_vel = movement_vel.normalize_or_zero();
        movement_vel *= PLAYER_SPEED;
        if input.pressed(KeyCode::ShiftLeft) {
            movement_vel *= PLAYER_SPRINT_BOOST;
        }
        movement_vel = camera.rotation * movement_vel;

        // Add to current velocity
        lin_vel.0 += movement_vel.adjust_precision();

        let current_speed = lin_vel.length();
        if current_speed > 0.0 {
            // Apply friction
            lin_vel.0 = lin_vel.0 / current_speed
                * (current_speed
                    - current_speed * PLAYER_SLOWDOWN_MULT * time.delta_secs().adjust_precision())
                .max(0.0)
        }
    }
}

fn debug_comamnds(
    mut player_tf_query: Query<&mut Transform, With<PlayerCamera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut player_tf in &mut player_tf_query {
        // reset player location to start transfrom
        if input.pressed(KeyCode::KeyR) {
            player_tf.translation = PLAYER_START_LOC.translation;
        }
    }
}

fn update_camera_transform(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut camera: Query<&mut Transform, With<PlayerCamera>>,
) {
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };

    let delta = accumulated_mouse_motion.delta;
    let delta_yaw = -delta.x * 0.003;
    let delta_pitch = -delta.y * 0.003;

    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;

    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Locked;
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
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

const SPRITE_ROTATE_THRESHOLD: f32 = 0.0001;

fn update_billboards(
    camera_query: Query<&Transform, (With<Camera3d>, Without<NPCSprite>)>,
    mut sprite_query: Query<&mut Transform, (With<NPCSprite>, Without<Camera3d>)>,
) {
    let Ok(cam_tf) = camera_query.single() else {
        return;
    };
    for mut sprite_tf in &mut sprite_query {
        // check diff between current sprite rotation and target
        let current_sprite_rotation = sprite_tf.rotation.clone();
        let target_tf = cam_tf.clone();
        let target_tf_rotation = target_tf.rotation;
        let diff = target_tf_rotation.angle_between(current_sprite_rotation);
        if diff > SPRITE_ROTATE_THRESHOLD {
            sprite_tf.rotation = target_tf.rotation;
        }
    }
}

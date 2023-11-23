
#[macro_use]
mod macros;
use macros::*;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{animation::RepeatAnimation, pbr::CascadeShadowConfigBuilder};

fn main() {
    App::new()
     //.add_plugins(DefaultPlugins)
     .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "freedom-tape".to_string(),
                decorations: false,
                ..Default::default()
            }),
            ..Default::default()
        })
      )
     .add_systems(Update, on_update_system)
     .insert_resource(AmbientLight {
          color: Color::WHITE,
          brightness: 1.0,
      })
      .add_systems(Startup, setup)
      .add_systems(
          Update,
          (setup_scene_once_loaded, keyboard_animation_control),
      )
     .run();
}

static HAVE_FLOATED_WINDOW: AtomicBool = AtomicBool::new(false);
static UPDATE_TICK: AtomicUsize = AtomicUsize::new(0);

fn on_update_system() {
  let tick = UPDATE_TICK.load(Ordering::Relaxed);

  if !HAVE_FLOATED_WINDOW.load(Ordering::Relaxed) {
    maybe_tell_window_manager_to_float_us();
  }

  UPDATE_TICK.fetch_add(1, Ordering::Relaxed);
}


fn maybe_tell_window_manager_to_float_us() {
  if let Ok(sway_sock_ipc) = std::env::var("SWAYSOCK") {
    if let Ok(mut connection) = swayipc::Connection::new() {
      // First; scan for freedom-tape window.
      if let Ok(win_tree) = connection.get_tree() {
        if let Some(freedom_tape_node) = lookup_node(win_tree, "freedom-tape") {
          dump_error!(connection.run_command("[title=\"^freedom-tape$\"] floating enable"));
          HAVE_FLOATED_WINDOW.store(true, Ordering::SeqCst);
        }
      }
    }
  }
  else {
    // No SWAYSOCK in env, stop checking
    HAVE_FLOATED_WINDOW.store(true, Ordering::SeqCst);
  }
}

fn lookup_node(root: swayipc::Node, name_to_find: &str) -> Option<swayipc::Node> {
  let name = root.name.clone().unwrap_or(String::new());
  if &name == name_to_find {
    return Some(root.clone());
  }
  for child in root.nodes {
    if let Some(found_node) = lookup_node(child, name_to_find) {
      return Some(found_node.clone());
    }
  }
  return None;
}


#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("/tmp/Fox.glb#Animation2"),
        asset_server.load("/tmp/Fox.glb#Animation1"),
        asset_server.load("/tmp/Fox.glb#Animation0"),
    ]));

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100.0, 100.0, 150.0)
            .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
        ..default()
    });

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500000.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -std::f32::consts::PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // Fox
    commands.spawn(SceneBundle {
        scene: asset_server.load("/tmp/Fox.glb#Scene0"),
        ..default()
    });

    println!("Animation controls:");
    println!("  - spacebar: play / pause");
    println!("  - arrow up / down: speed up / slow down animation playback");
    println!("  - arrow left / right: seek backward / forward");
    println!("  - digit 1 / 3 / 5: play the animation <digit> times");
    println!("  - L: loop the animation forever");
    println!("  - return: change animation");
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    for mut player in &mut animation_players {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    std::time::Duration::from_millis(250),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Key1) {
            player.set_repeat(RepeatAnimation::Count(1));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Key3) {
            player.set_repeat(RepeatAnimation::Count(3));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Key5) {
            player.set_repeat(RepeatAnimation::Count(5));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::L) {
            player.set_repeat(RepeatAnimation::Forever);
        }
    }
}

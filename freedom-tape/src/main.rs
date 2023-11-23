
#[macro_use]
mod macros;
use macros::*;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

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
     .add_systems(Startup, setup)
     .run();
}

static HAVE_FLOATED_WINDOW: AtomicBool = AtomicBool::new(false);
static UPDATE_TICK: AtomicUsize = AtomicUsize::new(0);

fn on_update_system() {
  println!("hello world");
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


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}

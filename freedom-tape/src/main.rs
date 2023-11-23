
#[macro_use]
mod macros;
use macros::*;

use std::sync::atomic::{AtomicUsize, Ordering};

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
     .add_systems(Update, hello_world_system)
     .add_systems(Startup, setup)
     .run();
}

static UPDATE_TICK: AtomicUsize = AtomicUsize::new(0);

fn hello_world_system() {
  println!("hello world");
  let tick = UPDATE_TICK.load(Ordering::Relaxed);
  if tick > 6 && tick < 8 {
    maybe_tell_window_manager_to_float_us();
  }
  UPDATE_TICK.fetch_add(1, Ordering::Relaxed);
}


fn maybe_tell_window_manager_to_float_us() {
  if let Ok(sway_sock_ipc) = std::env::var("SWAYSOCK") {
    if let Ok(mut connection) = swayipc::Connection::new() {
      //dump_error!(connection.run_command("for_window[title=\"freedom-tape\"] floating enable"));
      dump_error!(connection.run_command("[title=\"freedom-tape\"] focus ; floating enable"));
    }
  }
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


#[macro_use]
mod macros;
use macros::*;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use godot::prelude::*;
use godot::engine::Sprite2D;
use godot::engine::ISprite2D;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    angular_speed: f64,

    #[base]
    sprite: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(sprite: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            sprite
        }
    }
    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        self.sprite.rotate((self.angular_speed * delta) as f32);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }

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


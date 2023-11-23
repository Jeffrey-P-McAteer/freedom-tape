
# Freedom Tape

Freedom Tape is a videogame exploring how simple decisions affect complex worlds;
you play as the leader of a society whose only goal is to effect peaceful changes
as chaos appears in your world.

![graphics/icon.128.png](graphics/icon.128.png)


## Building

```bash
cargo build --release

```

## Testing

```bash
TODO
```

## Useful diagnostics

```bash
# See https://github.com/gfx-rs/wgpu#environment-variables
RUST_LOG=info WINIT_UNIX_BACKEND=wayland WGPU_BACKEND=gl cargo run --release


# GPU software that didn't fix Bevy render issues on Wayland
yay -S vulkan-radeon vulkan-intel vulkan-mesa-layers \
  mesa-vulkan-drivers  vulkan-icd-loader \
   vulkan-swrast

LIBGL_ALWAYS_SOFTWARE=1 __GLX_VENDOR_LIBRARY_NAME=mesa VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.i686.json:/usr/share/vulkan/icd.d/lvp_icd.x86_64.json RUST_LOG=info WINIT_UNIX_BACKEND=wayland WGPU_BACKEND=gl cargo run --release

# ^^ That one worked!


```


## License

```
freedom-tape videogame
Copyright (C) 2023 Jeffrey McAteer <jeffrey@jmcateer.com>

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; only version 2
of the License, and never any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
```

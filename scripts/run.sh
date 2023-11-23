#!/bin/sh

set -e


BUILD_CMD="$@"
if test -z "$BUILD_CMD" ; then
  BUILD_CMD='cargo build --release'
fi

if test "$HOSTNAME" = "azure-angel" ; then
  # if test -e /dev/nvidiactl ; then
  #   export __NV_PRIME_RENDER_OFFLOAD=1
  #   export __GLX_VENDOR_LIBRARY_NAME=nvidia
  #   export DRI_PRIME=1
  #   export WINIT_UNIX_BACKEND=wayland
  #   export WGPU_ADAPTER_NAME=nvidia
  # else
  #   export LIBGL_ALWAYS_SOFTWARE=1
  #   export __GLX_VENDOR_LIBRARY_NAME=mesa
  #   export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.i686.json:/usr/share/vulkan/icd.d/lvp_icd.x86_64.json
  #   export WINIT_UNIX_BACKEND=wayland
  # fi
  echo nop
fi

$BUILD_CMD

# Run the game
if test -z "$@" ; then
  godot --path ./godot --rendering-driver opengl3
fi

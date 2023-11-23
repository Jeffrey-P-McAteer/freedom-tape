#!/bin/sh

set -e


CMD="$@"
if test -z "$CMD" ; then
  CMD='cargo run --release'
fi

if test "$HOSTNAME" = "azure-angel" ; then
  export LIBGL_ALWAYS_SOFTWARE=1
  export __GLX_VENDOR_LIBRARY_NAME=mesa
  export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.i686.json:/usr/share/vulkan/icd.d/lvp_icd.x86_64.json
  export WINIT_UNIX_BACKEND=wayland
fi

$CMD


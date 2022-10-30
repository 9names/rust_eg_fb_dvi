Toy project using Rust Embedded-Graphics with pico-sdk and libdvi.
Haven't worked out how to build outside of the libdvi repo, so for now that's the only supported build option.

Usage:
Install Corrosion following instructions from https://github.com/corrosion-rs/corrosion
```system
export PICO_SDK_PATH=[insert your Pico SDK path here]
git clone https://github.com/Wren6991/picodvi
cd picodvi/software/apps
git clone https://github.com/9names/rust_eg_fb_dvi
echo 'add_subdirectory(rust_eg_fb_dvi)' >> CMakeLists.txt
cd ..
cmake -Btarget -GNinja
cmake --build target
elf2uf2-rs -d target/apps/rust_eg_fb_dvi/rust_embedded_graphics.elf
```
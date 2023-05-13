Toy project using Rust Embedded-Graphics with pico-sdk and libdvi.
Haven't worked out how to build outside of the libdvi repo, so for now that's the only supported build option.

Note: cbindgen doesn't quite build rust-eg-fb.h correctly - don't run rebuild.sh unless you're willing to manually fix it

Usage:
```system
export PICO_SDK_PATH=[insert your Pico SDK path here]
git clone https://github.com/Wren6991/picodvi
cd picodvi/software
git clone https://github.com/9names/rust_eg_fb_dvi ./apps/rust_eg_fb_dvi
echo 'add_subdirectory(rust_eg_fb_dvi)' >> ./apps/CMakeLists.txt
cmake -Bbuilddir -GNinja
cmake --build builddir
elf2uf2-rs -d builddir/apps/rust_eg_fb_dvi/rust_embedded_graphics.elf
```

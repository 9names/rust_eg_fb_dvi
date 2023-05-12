# Replace TMDS with 10 bit UART (same baud rate):
# add_definitions(-DDVI_SERIAL_DEBUG=1)
# add_definitions(-DRUN_FROM_CRYSTAL)

set(Rust_CARGO_TARGET thumbv6m-none-eabi)
include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.3.5 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH rust-eg-fb/Cargo.toml)

add_executable(rust_embedded_graphics
	main.c
)

target_compile_options(rust_embedded_graphics PRIVATE -Wall)

target_compile_definitions(rust_embedded_graphics PRIVATE
	DVI_DEFAULT_SERIAL_CONFIG=${DVI_DEFAULT_SERIAL_CONFIG}
	)

target_link_libraries(rust_embedded_graphics
	pico_stdlib
	pico_multicore
	pico_util
	libdvi
	rust-eg-fb
)

# create map/bin/hex file etc.
pico_add_extra_outputs(rust_embedded_graphics)

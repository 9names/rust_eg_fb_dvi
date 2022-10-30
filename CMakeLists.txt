# Replace TMDS with 10 bit UART (same baud rate):
# add_definitions(-DDVI_SERIAL_DEBUG=1)
# add_definitions(-DRUN_FROM_CRYSTAL)

set(Rust_CARGO_TARGET thumbv6m-none-eabi)
find_package(Corrosion REQUIRED)

corrosion_import_crate(MANIFEST_PATH rust-eg-fb/Cargo.toml
	NO_STD
	PROFILE release
)

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

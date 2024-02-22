host := `uname -a`

# print this message
help:
    just --list

# run specified example
run EXAMPLE:
    cargo r --package bevy_venx --features "dyn" --example {{EXAMPLE}}

# run example with release flag
run-release EXAMPLE:
    cargo r --release --package bevy_venx --example {{EXAMPLE}}

# profile given example with flamegraph
profile EXAMPLE:
    CARGO_PROFILE_RELEASE_DEBUG=true RUSTFLAGS='-C force-frame-pointers=y' cargo flamegraph -c "record -g" --package bevy_venx --example {{EXAMPLE}}
    
# build project and compile shaders in release mod and output compile timings in `target/cargo-timings`
build:
    cargo build --timings --release

# print available examples
examples:
    echo "Not done yeeet"

# run demo
demo:
    cargo r --release --package bevy_venx --bin bevy

# install all deps for debian
deps:
     echo "Not done yeeet"

# display lines of code
lines:
    find . -name '*.rs' | xargs wc -l

# clean cached files
clean:
     echo "Not done yeeet"

# cargo check
check:
    cargo check
# test specific package in project
test PACKAGE:
    cargo test --package {{PACKAGE}}

# test just venx crate
test-venx:
    cargo test

# compile shader in wgsl and output in target/venx_shaders.wgsl
wgsl:
    cargo b --package bevy_venx --features "dyn"
    naga target/spirv-builder/spirv-unknown-vulkan1.2/release/deps/venx_shaders.spv target/venx_shaders.wgsl

# compile shader in wgsl and output in target/{{NAME}}.wgsl
wgsl-named NAME:
    cargo b --package bevy_venx --features "dyn"
    naga target/spirv-builder/spirv-unknown-vulkan1.2/release/deps/venx_shaders.spv target/{{NAME}}.wgsl
    
# compile shader in metal and output in target/venx_shaders.metal
metal:
    cargo b --package bevy_venx --features "dyn"
    naga target/spirv-builder/spirv-unknown-vulkan1.2/release/deps/venx_shaders.spv target/venx_shaders.metal

# compile shader in metal and output in target/{{NAME}}.metal
metal-named NAME:
    cargo b --package bevy_venx --features "dyn"
    naga target/spirv-builder/spirv-unknown-vulkan1.2/release/deps/venx_shaders.spv target/{{NAME}}.metal
    
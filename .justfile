
# print this message
help:
    just --list

# run specified example
run EXAMPLE="":
    cargo r --package bevy_venx --features "dyn" --example {{EXAMPLE}}

# convert supported format into .plat
convert PATH FORMAT="mca" OUT=".cache":
    if "{{FORMAT}}" != "mca" { "Unsupported format ({{FORMAT}})" } else { "TODO" }

# Open .plat in basic environment
open PATH="plats/basic.plat":
    echo "TODO"
    # TODO: Open in demo?

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
    ls bevy_venx/examples | sed -e 's/\.rs$//'

# run demo
demo: (run "demo")

# install all deps for debian
[confirm("Do you really want to instal additional dependencies on your system?")]
deps:
     echo "Not done yeeet"

# display lines of code
lines:
    find . -name '*.rs' | xargs wc -l

# clean cached files
[confirm("It will erase everything in .cache and target dirs. Are you sure?")]
clean:
     echo "Not done yeeet"

# test specific package in project. Default is venx
test +PACKAGE='venx':
    cargo test --package {{PACKAGE}}

# clippy --fix specified package
clippy +PACKAGE='venx':
    cargo clippy --fix --package {{PACKAGE}} 

_to_shader TARGET='wgsl' NAME='venx_shaders' SPV_V='1.1':
    cargo b --package venx_shaders
    naga target/spirv-builder/spirv-unknown-vulkan{{SPV_V}}/release/deps/venx_shaders.spv target/{{NAME}}.{{TARGET}}

# compile shader in wgsl
wgsl NAME='venx_shaders' SPV_V='1.2': (_to_shader "wgsl" NAME SPV_V)

# compile shader in metal
metal NAME='venx_shaders' SPV_V='1.2': (_to_shader "metal" NAME SPV_V)

# watch any command. Example: just watch run turbo
watch +CMDS:
    cargo-watch -s "just {{CMDS}}"

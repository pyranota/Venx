host := `uname -a`

# print this message
help:
    just --list

# run specified example
run EXAMPLE:
    echo "Hello" {{EXAMPLE}}

# run demo without release flag
dev:
    cargo r --package bevy_venx --bin bevy
    
# build project and compile shaders
build:
    cargo build 

# print available examples
examples:
    echo "1"

# run demo
demo:
    cargo r --release --package bevy_venx --bin bevy

# install all deps for debian
deps:
    echo "Installing dependencies"

# display lines of code
lines:
    find . -name '*.rs' | xargs wc -l

# clean cached files
clean:
    echo "Cleaning cached files"

# check if everything installed and configured correctly
check:
    echo "Checking system"

# test specific package in project
test PACKAGE:
    cargo test --package {{PACKAGE}}

# test just venx crate
test-venx:
    cargo test

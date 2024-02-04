host := `uname -a`

# print this message
help:
    just --list

# run specified example
run EXAMPLE:
    echo "Hello" {{EXAMPLE}}

# print available examples
examples:
    echo "1"

# run demo
demo:
    echo "Running Demo"

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
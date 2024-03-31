#! /bin/env bash

# In rare cases `blake3` rust dependency recompiles each time which is annoying
# To fix it export these env vars
# run `. ./fix_blake3.sh` or `source ./fix_blake3.sh`
export CC=gcc
export AR=ar

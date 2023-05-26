#!/bin/bash

path="/tmp/stdselect"
destination="/usr/local/bin/stdselect"

function compile_binary () {
    echo "[INFO] Building binary ✅" && \
    CARGO_TARGET_DIR=${path} cargo build --release 2> /dev/null
}

function place_binary_in_bin () {
    echo "[INFO] Moving binary to $destination ✅" && \
    mv ${path}/release/stdselect ${destination}
}

function check_binary () {
    binary=$(ls ${destination})
    if [ $? -eq 0 ]; then
        echo "[DONE] Installation complete ✅"
    else
        echo "[ERROR] Installation failed ❌"
    fi
}

compile_binary && \
place_binary_in_bin && \
check_binary

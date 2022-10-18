#!/bin/bash

cargo_status=`ls /home/$USER/.cargo/bin/ | grep "cargo$"`
if [[ $cargo_status == "" ]];then
    echo "Cargo is not installed!"
    echo "Please run: 'curl https://sh.rustup.rs -sSf | sh'"
    exit 1
fi

cargo build --release
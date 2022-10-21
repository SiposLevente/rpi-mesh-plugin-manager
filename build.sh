#!/bin/bash
cd `dirname $0`
cargo_status="`ls ~/.cargo/bin/ | grep "cargo$"`"
if [[ $cargo_status == "" ]];then
    echo "Cargo is not installed!"
    echo "Please run: 'curl https://sh.rustup.rs -sSf | sh'"
    exit 1
fi

libssl_is_installed="`dpkg -l | grep libssl-dev`"
if [[ $libssl_is_installed == "" ]];then
    echo "Libssl-dev is not installed! Installing..."
    sudo apt install -y libssl-dev
    if [ $? == 0 ];then
        successful=1
    fi
else
    successful=1
fi

if [ $successful == 1 ];then
    cargo build --release
fi
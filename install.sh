#!/bin/bash

if [[ `whoami` != "root" ]];then
    echo "Run this script with root privilages!"
    exit 1
fi

target=`ls ./target/release | grep "rpi-mesh-plugin-manager"`
if [[ $target == "" ]];then
    echo "Plugin manager is not built! Please run 'build.sh' first!"
    exit 1
fi

cp ./target/release/rpi-mesh-plugin-manager /bin/
mkdir -p /etc/rpi-mesh-plugin-manager
cp ./plugins.repo /etc/rpi-mesh-plugin-manager
#!/bin/bash

appname='tetris'

# compile rust project  # TODO
    #rustc tetrust/src/main.rs

rust_binary_file="tetrust/target/release/tetrust"
target_package_binary="$appname/usr/bin/$appname"
cp "$rust_binary_file" "$target_package_binary"



ver=$(grep '^Version: ' "$appname/DEBIAN/control" | cut -d' ' -f2)
filename="${appname}-${ver}.deb"
dpkg-deb --build "$appname" "$filename"

echo "build done, install by running: \"sudo dpkg -i $filename\""
echo "you can later purge it by running: \"sudo dpkg -P $appname\""


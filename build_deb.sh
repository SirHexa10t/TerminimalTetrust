#!/bin/bash

appname='tetrinal'
ver=$(grep '^Version: ' "$appname/DEBIAN/control" | cut -d' ' -f2)
filename="${appname}-${ver}.deb"
dpkg-deb --build "$appname" "$filename"

echo "build done, install by running: \"sudo dpkg -i $filename\""
echo "you can later purge it by running: \"sudo dpkg -P $appname\""


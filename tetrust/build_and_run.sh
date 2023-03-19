#!/bin/bash

cargo build

read -n 1 -p "Press any key to continue" in
target/debug/tetrust

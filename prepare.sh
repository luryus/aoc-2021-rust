#!/bin/bash

# cd to the script dir
cd "${0%/*}"

day="$1"

./input.py "$day"
cp -n "src/bin/template.rs_" "src/bin/$(printf "%02d" "$day").rs"
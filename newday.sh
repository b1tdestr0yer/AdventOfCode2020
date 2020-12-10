#!/bin/bash

echo "Making day $1 and challenge $2";

mkdir "Day$1"
cd "Day$1"
mkdir "$2"
cd "$2"
curl -b "$SESSIONAOC" "https://adventofcode.com/2020/day/$1/input" > input
if [ "$2" -eq "1" ]; then
    cp "../../template.rs" "main.rs"
else
    cp "../1/main.rs" "main.rs"
fi

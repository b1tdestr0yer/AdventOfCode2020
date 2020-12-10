#!/bin/bash

echo "Making day $1 and challenge $2";

mkdir "Day$1"
cd "Day$1"
mkdir "$2"
cd "$2"
curl -b "$SESSIONAOC" "https://adventofcode.com/2020/day/$1/input" > input
cp "../../template.rs" "main.rs"

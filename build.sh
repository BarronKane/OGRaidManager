#!/bin/bash

set -e

cd "`dirname "$0"`"

mkdir -p target/debug
mkdir -p target/release

cp resources/* target/debug/
cp resources/* target/release/

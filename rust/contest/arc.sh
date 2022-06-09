#!/bin/bash

contest=$1
problem=$2

echo "Archive to $contest-$problem"

mkdir -p ../$contest
cp -i ./src/main.rs ../$contest/$problem.rs

echo "done!🍻"

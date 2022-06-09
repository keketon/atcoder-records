#!/bin/bash

COLOR="\e[34m"
COLOR_OFF="\e[m"

contest=$1
problem=$2

echo -e "${COLOR}Archive and commit [$contest-$problem]${COLOR_OFF}"

mkdir -p ../$contest
cp -i ./src/main.rs ../$contest/$problem.rs

echo -e "${COLOR}cp completed${COLOR_OFF}"

git add ../$contest/$problem.rs
git commit -m "$contest-$problem"

echo -e "${COLOR}Commit completed${COLOR_OFF}"

echo -e "\e[32;1mAll done!🍻${COLOR_OFF}"

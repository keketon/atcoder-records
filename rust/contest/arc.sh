#!/bin/bash

USAGE="Usage: './arc.sh abc123 a'"

COLOR_INFO="\e[34m"
COLOR_OK="\e[32;1m"
COLOR_ERR="\e[31m"
COLOR_OFF="\e[m"

contest=$1
problem=$2

if [ -z "$contest" ] || [ -z "$problem" ]; then
  echo -e "${COLOR_ERR}Illegal arguments.${COLOR_END}"
  echo -e "${COLOR_ERR}$USAGE${COLOR_END}"
  exit 1
fi

echo -e "${COLOR_INFO}Archive and commit [$contest-$problem]${COLOR_OFF}"

mkdir -p ../$contest
cp -i ./src/main.rs ../$contest/$problem.rs

echo -e "${COLOR_INFO}cp completed${COLOR_OFF}"

git add ../$contest/$problem.rs
git commit -m "$contest-$problem"

echo -e "${COLOR_INFO}Commit completed${COLOR_OFF}"

echo -e "${COLOR_OK}All done!🍻${COLOR_OFF}"

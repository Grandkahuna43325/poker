#!/bin/bash

# Check if two arguments are provided
if [ $# -ne 2 ]; then
  echo "give url1 and url2"
  exit 1
fi

string1=$1
string2=$2


find . -type f -exec sed -i 's|'$string1'|'$string2'|g' {} +

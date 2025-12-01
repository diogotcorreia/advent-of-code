#!/usr/bin/env sh

set -e

YEAR=2025

SCRIPT_DIR=$(dirname -- "$0")

if [ -z "$AOC_SESSION" ]; then
  echo "error: AOC_SESSION not defined" >&2;
  exit 1
fi

re='^([1-9][0-9]*)|0$'
if ! [[ $1 =~ $re ]] ; then
   echo "error: day is not an integer" >&2
   echo >&2
   echo "usage: $0 <day>" >&2
   exit 1
fi

curl "https://adventofcode.com/$YEAR/day/$1/input" -H "Cookie: session=$AOC_SESSION" -o "$SCRIPT_DIR/inputs/day$(printf "%02d" "$1").txt"

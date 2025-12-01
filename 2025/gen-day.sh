#!/usr/bin/env sh

set -e

SCRIPT_DIR=$(dirname -- "$0")

re='^([1-9][0-9]*)|0$'
if ! [[ $1 =~ $re ]] ; then
   echo "error: day is not an integer" >&2
   echo >&2
   echo "usage: $0 <day>" >&2
   exit 1
fi

day_no_padding="$(printf "%d" "$1")"
day="$(printf "%02d" "$1")"
target_file="$SCRIPT_DIR/src/day$day.rs"
template_file="$SCRIPT_DIR/src/day00.rs.template"
main_file="$SCRIPT_DIR/src/main.rs"

if [[ -a "$target_file" ]]; then
   echo "error: file $target_file already exists"
   exit 1
fi

cp "$template_file" "$target_file"
sed -i "s/00/$day/g" "$target_file"
sed -i "s/\/\/ mod day$day;/mod day$day;/" "$main_file" || true
sed -i "s/\/\/ $day_no_padding => run_day/$day_no_padding => run_day/" "$main_file" || true

echo "file $target_file generated successfully"

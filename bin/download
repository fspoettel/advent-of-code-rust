#!/bin/bash

set -e;

if ! command -v 'aoc' &> /dev/null
then
    echo "command \`aoc\` not found. Try running \`cargo install aoc-cli\` to install it."
    exit 1
fi

POSITIONAL=()
while [[ $# -gt 0 ]]; do
  key="$1"

  case $key in
    -y|--year)
      year="$2"
      shift
      shift
      ;;
    *)
      POSITIONAL+=("$1")
      shift
      ;;
  esac
done

set -- "${POSITIONAL[@]}"

if [ -z "$1" ]; then
    >&2 echo "Argument is required for day."
    exit 1
fi

day=$(echo "$1" | sed 's/^0*//');
day_padded=$(printf %02d "$day");

filename="$day_padded";
input_path="src/inputs/$filename.txt";

tmp_dir=$(mktemp -d);
tmp_file_path="$tmp_dir/input";

if [[ "$year" != "" ]]
then
aoc download --day "$day" --year "$year" --file "$tmp_file_path";
else
aoc download --day "$day" --file "$tmp_file_path";
fi

cat "$tmp_file_path" > "$input_path";
echo "---"
echo "🎄 Successfully wrote input to \"$input_path\"!"

trap "exit 1"            HUP INT PIPE QUIT TERM
trap 'rm -rf "$tmp_dir"' EXIT

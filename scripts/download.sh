#!/bin/bash

set -e;

if ! command -v 'aoc' &> /dev/null
then
    echo "command \`aoc\` not found. Try running \`cargo install aoc-cli\` to install it."
    exit 1
fi

if [ ! -n "$1" ]; then
    >&2 echo "Argument is required for day."
    exit 1
fi

day=$(echo $1 | sed 's/^0*//');
day_padded=`printf %02d $day`;

filename="day$day_padded";
input_path="src/inputs/$filename.txt";

tmp_dir=$(mktemp -d);
tmp_file_path="$tmp_dir/input";

aoc download --day $day --file $tmp_file_path;
cat $tmp_file_path > $input_path;
echo "Wrote input to \"$input_path\"...";

cat <<EOF
   _==_ _
 _,(",)|_|
  \/. \-|
__( :  )|_  Done!
EOF

# Make sure it gets removed even if the script exits abnormally.
trap "exit 1"           HUP INT PIPE QUIT TERM
trap 'rm -rf "$tmp_dir"' EXIT

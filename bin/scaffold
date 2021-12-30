#!/bin/bash

set -e;

if [ -z "$1" ]; then
    >&2 echo "Argument is required for day."
    exit 1
fi

day=$(echo "$1" | sed 's/^0*//');
day_padded=$(printf %02d "$day");

filename="$day_padded";

input_path="src/inputs/$filename.txt";
example_path="src/examples/$filename.txt";
module_path="src/bin/$filename.rs";

touch "$module_path";

cat > "$module_path" <<EOF
pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

fn main() {
    aoc::solve!(&aoc::read_file("inputs", DAYNUM), part_one, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", DAYNUM);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", DAYNUM);
        assert_eq!(part_two(&input), 0);
    }
}
EOF

perl -pi -e "s,DAYNUM,$day,g" "$module_path";

echo "Created module \"$module_path\"";

touch "$input_path";
echo "Created empty input file \"$input_path\"";

touch "$example_path";
echo "Created empty example file \"$example_path\"";
echo "---"
echo "🎄 Type \`cargo run --bin $day_padded\` to run your solution."

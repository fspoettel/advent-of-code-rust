#!/bin/bash

set -e;

if [ ! -n "$1" ]; then
    >&2 echo "Argument is required for day."
    exit 1
fi

day=$(echo $1 | sed 's/^0*//');
day_padded=`printf %02d $day`;

filename="day$day_padded";

input_path="src/inputs/$filename.txt";
example_path="src/examples/$filename.txt";
module_path="src/solutions/$filename.rs";

touch $module_path;

cat > $module_path <<EOF
pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", day);
        assert_eq!(part_one(&input), 0);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", day);
        assert_eq!(part_two(&input), 0);
    }
}
EOF

perl -pi -e "s,day,$day,g" $module_path;

echo "Created module \"$module_path\"";

touch $input_path;
echo "Created input file \"$input_path\"";

touch $example_path;
echo "Created example file \"$example_path\"";

line="        $day => solve_day!($filename, &input),"
perl -pi -le "print '$line' if(/^*.day not solved/);" "src/main.rs";

echo "Linked new module in \"src/main.rs\"";

LINE="pub mod $filename;";
FILE="src/solutions/mod.rs";
grep -qF -- "$LINE" "$FILE" || echo "$LINE" >> "$FILE";
echo "Linked new module in \"$FILE\"";


cat <<EOF
   _==_ _
 _,(",)|_|
  \/. \-|
__( :  )|_  Done!
EOF

use std::fs::read_to_string;

pub fn execute(file_path: &str) -> usize {
    let disk_map = read_to_string(file_path).unwrap();

    checksum(&disk_map)
}

fn checksum(disk_map: &str) -> usize {
    let mut values: Vec<u32> = disk_map
        .chars()
        .filter(|it| !it.is_whitespace())
        .map(|it| it.to_digit(10).unwrap())
        .collect();

    let mut left_file = 0;
    let mut right_file = ((disk_map.len() + 1) / 2) - 1;

    let mut l = 0;
    let mut r = values.len() - 1;

    let mut result = 0;

    let mut pos = 0;

    while l < r {
        if l % 2 == 0 {
            while values[l] != 0 {
                result += pos * left_file;

                pos += 1;
                values[l] -= 1;
            }
            left_file += 1;
        } else {
            while values[l] != 0 {
                if values[r] != 0 {
                    result += pos * right_file;

                    pos += 1;
                    values[r] -= 1;
                    values[l] -= 1;
                } else {
                    r -= 2;
                    right_file -= 1;
                }
            }
        }

        l += 1;
    }

    while values[r] != 0 {
        result += pos * right_file;

        values[r] -= 1;
        pos += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::nine::first::checksum;

    #[test]
    fn first() {
        let input = "2333133121414131402";

        assert_eq!(1928, checksum(input));
    }
}

use std::{collections::VecDeque, fs::read_to_string};

#[derive(Clone, Copy, Debug)]
enum Block {
    // (block_size)
    Free(u32),
    // (file_id, block_size)
    Occupied(usize, u32),
}

pub fn execute(file_path: &str) -> usize {
    let disk_map = read_to_string(file_path).unwrap();

    checksum(&disk_map)
}

fn checksum(disk_map: &str) -> usize {
    let values: Vec<u32> = disk_map
        .chars()
        .filter(|it| !it.is_whitespace())
        .map(|it| it.to_digit(10).unwrap())
        .collect();

    let mut blocks = VecDeque::new();

    let mut file_id = 0;
    (0..values.len()).for_each(|i| {
        if i % 2 == 0 {
            blocks.push_back(Block::Occupied(file_id, values[i]));

            file_id += 1;
        } else {
            blocks.push_back(Block::Free(values[i]));
        }
    });

    let mut r = blocks.len() - 1;
    while r > 0 {
        if let Block::Occupied(id, size) = blocks[r] {
            let mut l = 1;

            while l < r {
                if let Block::Free(free_size) = blocks[l] {
                    if free_size < size {
                        l += 1;
                        continue;
                    }

                    let remaining = free_size - size;

                    blocks[l] = Block::Occupied(id, size);
                    blocks[r] = Block::Free(size);
                    if remaining != 0 {
                        blocks.insert(l + 1, Block::Free(remaining));
                    }
                    break;
                }
                l += 1;
            }
        }

        r -= 1;
    }

    let mut pos = 0;
    let mut checksum = 0;
    for block in blocks {
        match block {
            Block::Free(size) => pos += size as usize,
            Block::Occupied(id, size) => {
                for _ in 0..size {
                    checksum += pos * id;
                    pos += 1;
                }
            }
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use crate::nine::second::checksum;

    #[test]
    fn first() {
        let input = "2333133121414131402";

        assert_eq!(2858, checksum(input));
    }
}

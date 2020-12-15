use std::collections::HashMap;

pub fn solve_1() -> String {
    elf_1(&PUZZLE_INPUT, 2020).to_string()
}

pub fn solve_2() -> String {
    elf_1(&PUZZLE_INPUT, 30000000).to_string()
}

pub fn elf_1(starting_nums: &[usize], nth: usize) -> usize {
    let mut num2last = HashMap::new();
    let mut spoken = 0;
    for i in 0..nth {
        spoken = if i < starting_nums.len() {
            starting_nums[i]
        } else {
            let (turn, prev) = num2last.get(&spoken).unwrap();
            turn - prev
        };
        if let Some(entry) = num2last.get_mut(&spoken) {
            *entry = (i, entry.0);
        } else {
            num2last.insert(spoken, (i, i));
        }
    }
    spoken
}

const PUZZLE_INPUT: [usize; 7] = [2,0,1,7,4,14,18];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        for (nums, target) in [
            ([0,3,6], 436),
            ([1,3,2], 1),
            ([2,1,3], 10),
            ([1,2,3], 27),
            ([2,3,1], 78),
            ([3,2,1], 438),
            ([3,1,2], 1836)
        ].iter() {
            assert_eq!(elf_1(nums, 2020), *target);
        }
    }
}
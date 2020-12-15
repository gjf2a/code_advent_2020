use std::collections::HashMap;

pub fn solve_1() -> String {
    ElfGame::elf(&PUZZLE_INPUT, 2020).to_string()
}

pub fn solve_2() -> String {
    ElfGame::elf(&PUZZLE_INPUT, 30000000).to_string()
}

struct ElfGame {
    num2last: HashMap<usize,(usize,usize)>,
    spoken: usize
}

impl ElfGame {
    pub fn elf(starting_nums: &[usize], nth: usize) -> usize {
        let mut game = ElfGame {num2last: HashMap::new(), spoken: 0};
        for i in 0..nth {
            game.update_spoken(i, starting_nums);
            game.update_entry(i);
        }
        game.spoken
    }

    fn update_spoken(&mut self, i: usize, starting_nums: &[usize]) {
        self.spoken = if i < starting_nums.len() {
            starting_nums[i]
        } else {
            let (turn, prev) = self.num2last.get(&self.spoken).unwrap();
            turn - prev
        };
    }

    fn update_entry(&mut self, i: usize) {
        if let Some(entry) = self.num2last.get_mut(&self.spoken) {
            *entry = (i, entry.0);
        } else {
            self.num2last.insert(self.spoken, (i, i));
        }
    }
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
            assert_eq!(ElfGame::elf(nums, 2020), *target);
        }
    }
}
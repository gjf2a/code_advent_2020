use advent_code_lib::file2nums;
use std::io;
use std::collections::VecDeque;

pub fn solve_1() -> io::Result<String> {
    Ok(find_failing_xmas_num(&file2nums("in/day9.txt")?, 25).unwrap().to_string())
}

pub fn solve_2() -> io::Result<String> {
    Ok(find_encryption_weakness(&file2nums("in/day9.txt")?, 25).unwrap().to_string())
}

fn find_encryption_weakness(nums: &Vec<isize>, preamble_length: usize) -> Option<isize> {
    match find_failing_xmas_num(nums, preamble_length) {
        None => None,
        Some(failing) => {
            for num in 0..nums.len() {
                match find_contiguous_sequence_pair(nums, num, failing) {
                    None => {},
                    Some((smallest, largest)) => return Some(smallest + largest)
                }
            }
            None
        }
    }
}

fn find_failing_xmas_num(nums: &Vec<isize>, preamble_length: usize) -> Option<isize> {
    let mut prev_preamble = VecDeque::new();
    for num in nums.iter() {
        if prev_preamble.len() == preamble_length {
            match find_pair_sum(&prev_preamble, *num) {
                None => {return Some(*num);},
                Some((_,_)) => {prev_preamble.pop_front();}
            }
        }
        prev_preamble.push_back(*num);
    }
    None
}

fn find_pair_sum(prev: &VecDeque<isize>, target: isize) -> Option<(isize,isize)> {
    for i in 0..prev.len() {
        for j in i+1..prev.len() {
            if prev[i] + prev[j] == target {
                return Some((prev[i], prev[j]))
            }
        }
    }
    None
}

fn find_contiguous_sequence_pair(nums: &Vec<isize>, start: usize, target: isize) -> Option<(isize,isize)> {
    let mut sum = 0;
    let mut min = nums[start];
    let mut max = nums[start];
    for i in start..nums.len() {
        sum += nums[i];
        min = if nums[i] < min {nums[i]} else {min};
        max = if nums[i] > max {nums[i]} else {max};
        if sum == target {
            return Some((min, max));
        } else if sum > target {
            return None;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_1() {
        assert_eq!(find_failing_xmas_num(&file2nums("in/day9_ex.txt").unwrap(), 5).unwrap(), 127);
    }

    #[test]
    fn test_puzzle_2() {
        assert_eq!(find_encryption_weakness(&file2nums("in/day9_ex.txt").unwrap(), 5).unwrap(), 62);
    }
}
use advent_code_lib::file2nums;
use std::io;
use std::collections::VecDeque;

pub fn solve_1() -> io::Result<String> {
    Ok(find_failing_xmas_num(&file2nums("day_9_input.txt")?, 25).unwrap().to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(find_failing_xmas_num(&file2nums("day_9_example.txt").unwrap(), 5).unwrap(), 127);
    }
}
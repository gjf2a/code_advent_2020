use advent_code_lib::all_lines;
use std::io;
use bits::BitArray;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (count1, count3) = count_jolt_jumps(filename)?;
    Ok((count1 * count3).to_string())
}

pub fn solve_2(filename: &str) -> io::Result<String> {
    Ok(count_arrangements(filename)?.to_string())
}

fn make_joltage_vec(filename: &str) -> io::Result<Vec<usize>> {
    let mut nums = vec![0];
    all_lines(filename)?
        .map(|line| line.parse::<usize>().unwrap())
        .for_each(|joltage| nums.push(joltage));
    nums.sort();
    nums.push(nums.last().unwrap() + 3);
    Ok(nums)
}

fn count_jolt_jumps(filename: &str) -> io::Result<(usize, usize)> {
    let nums = make_joltage_vec(filename)?;
    let mut count1 = 0;
    let mut count3 = 0;
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        if diff == 1 {count1 += 1;}
        if diff == 3 {count3 += 1;}
    }
    Ok((count1, count3))
}

fn jolt_jump_ok(nums: &[usize], first: usize, second: usize) -> bool {
    nums[second] <= nums[first] + 3
}

// Base case: Start and end only: 1
// Recursive case:
// - Find lowest index whose value is successor - 3 or higher.
// - Find all possibilities of current and predecessors
// - Multiply
// Implement with dynamic programming
fn count_arrangements(filename: &str) -> io::Result<usize> {
    let nums = make_joltage_vec(filename)?;
    let mut results = vec![1_usize];
    for i in 1..nums.len() - 1 {
        let last = find_last_stable(&nums, i);
        results.push(results[last] * num_valid_in_window(&nums, last+1, i));
    }
    Ok(*results.last().unwrap())
}

fn find_last_stable(nums: &[usize], index_to_remove: usize) -> usize {
    let mut j = index_to_remove - 1;
    while j > 0 && jolt_jump_ok(&nums, j, index_to_remove + 1) {
        j -= 1;
    }
    j
}

fn num_valid_in_window(nums: &[usize], start: usize, end: usize) -> usize {
    BitArray::all_combinations(end - start + 1).iter()
        .filter(|window| valid_window(nums, window, start))
        .count()
}

fn valid_window(nums: &[usize], keep_window: &BitArray, window_start: usize) -> bool {
    if window_start == 0 || window_start + keep_window.len() as usize >= nums.len() {return false;}
    let kept = kept_window(nums, keep_window, window_start);
    (1..kept.len()).all(|i| jolt_jump_ok(&kept, i-1, i))
}

fn kept_window(nums: &[usize], keep_window: &BitArray, window_start: usize) -> Vec<usize> {
    let mut num_window = vec![nums[window_start - 1]];
    (0..keep_window.len() as usize)
        .filter(|i| keep_window.is_set(*i as u64))
        .for_each(|i| num_window.push(nums[i + window_start]));
    num_window.push(nums[window_start + keep_window.len() as usize]);
    num_window
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1_1() {
        assert_eq!(count_jolt_jumps("in/day10_ex1.txt").unwrap(), (7, 5));
    }

    #[test]
    fn text_ex_2_1() {
        assert_eq!(count_jolt_jumps("in/day10_ex2.txt").unwrap(), (22, 10));
    }

    #[test]
    fn test_ex_1_2() {
        assert_eq!(count_arrangements("in/day10_ex1.txt").unwrap(), 8);
    }

    #[test]
    fn test_ex_2_2() {
        assert_eq!(count_arrangements("in/day10_ex2.txt").unwrap(), 19208);
    }

    #[test]
    fn test_self_1() {
        assert_eq!(count_arrangements("in/day10_self1.txt").unwrap(), 4);
    }

    #[test]
    fn test_count_in_window_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let keepers = BitArray::all_combinations(3);
        assert!(!valid_window(&nums, &keepers[0], 1));
        for i in 1..keepers.len() {
            assert!(valid_window(&nums, &keepers[i], 1));
        }

        assert_eq!(num_valid_in_window(&nums, 1, 3), 7);
    }

    #[test]
    fn test_count_in_window_2() {
        let nums = vec![0, 1, 2, 3, 6];
        assert_eq!(num_valid_in_window(&nums, 1, 2), 4);
        assert_eq!(num_valid_in_window(&nums, 2, 3), 2);
    }

    #[test]
    fn test_count_in_window_3() {
        let nums = vec![0, 1, 2, 3, 4, 5, 8];
        assert_eq!(num_valid_in_window(&nums, 1, 2), 4);

    }

    #[test]
    fn test_self_2() {
        // Possibilities:
        // 1, 2, 3, 4
        // 1, 2, 4
        // 1, 4
        // 1, 3, 4
        // 2, 4
        // 3, 4
        // 2, 3, 4
        assert_eq!(count_arrangements("in/day10_self2.txt").unwrap(), 7)
    }

    #[test]
    fn test_self_3() {
        // Possibilities:
        // 1, 2, 3, 4, 5
        // Delete 5: No
        // Delete 4: 1, 2, 3, 5; 1, 2, 5; 2, 5; 1, 3, 5; 3, 5 (total: 4)
        // Delete 3: 1, 2, 4, 5; 1, 4, 5; 2, 4, 5 (total: 3)
        // Delete 2: 1, 3, 4, 5; 3, 4, 5 (total: 2)
        // Delete 1: 2, 3, 4, 5 (total: 1)
        //
        // Turns out I undercounted by 3, as there are 14 rather than 11.
        assert_eq!(count_arrangements("in/day10_self3.txt").unwrap(), 14);
    }
}


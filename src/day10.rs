use advent_code_lib::all_lines;
use std::io;

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
        .map(|line| line.unwrap().parse::<usize>().unwrap())
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
        let mut j = 0;
        while j < i - 1 && !jolt_jump_ok(&nums, j + 1, i+1) {
            j += 1;
        }
        results.push(results[j] * count_in_window(&nums, j+1, i));
    }
    Ok(*results.last().unwrap())
}

fn count_in_window(nums: &[usize], start: usize, end: usize) -> usize {
    let mut count = 0;
    for window in all_windows(end - start + 1) {
        if valid_window(nums, &window, start) {
            count += 1;
        }
    }
    count
}

fn valid_window(nums: &[usize], keep_window: &[bool], window_start: usize) -> bool {
    if window_start == 0 || window_start + keep_window.len() >= nums.len() {return false;}
    let kept = kept_window(nums, keep_window, window_start);
    (1..kept.len()).all(|i| jolt_jump_ok(&kept, i-1, i))
}

fn kept_window(nums: &[usize], keep_window: &[bool], window_start: usize) -> Vec<usize> {
    let mut num_window = vec![nums[window_start - 1]];
    for i in 0..keep_window.len() {
        if keep_window[i] {
            num_window.push(nums[i + window_start]);
        }
    }
    num_window.push(nums[window_start + keep_window.len()]);
    num_window
}

fn all_windows(size: usize) -> Vec<Vec<bool>> {
    if size == 0 {
        vec![vec![]]
    } else {
        let mut result = Vec::new();
        for mut candidate in all_windows(size - 1) {
            let mut candidate1 = candidate.clone();
            candidate1.push(false);
            result.push(candidate1);
            candidate.push(true);
            result.push(candidate);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1_1() {
        assert_eq!(count_jolt_jumps("day_10_example_1.txt").unwrap(), (7, 5));
    }

    #[test]
    fn text_ex_2_1() {
        assert_eq!(count_jolt_jumps("day_10_example_2.txt").unwrap(), (22, 10));
    }

    #[test]
    fn test_ex_1_2() {
        assert_eq!(count_arrangements("day_10_example_1.txt").unwrap(), 8);
    }

    #[test]
    fn test_ex_2_2() {
        assert_eq!(count_arrangements("day_10_example_2.txt").unwrap(), 19208);
    }

    #[test]
    fn test_self_1() {
        assert_eq!(count_arrangements("day_10_self_example_1.txt").unwrap(), 4);
    }

    #[test]
    fn test_all_windows() {
        assert_eq!(all_windows(1), vec![vec![false], vec![true]]);
        assert_eq!(all_windows(2), vec![vec![false, false], vec![false, true], vec![true, false], vec![true, true]]);
    }

    #[test]
    fn test_count_in_window_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let keepers = all_windows(3);
        assert!(!valid_window(&nums, &keepers[0], 1));
        for i in 1..keepers.len() {
            assert!(valid_window(&nums, &keepers[i], 1));
        }

        assert_eq!(count_in_window(&nums, 1, 3), 7);
    }

    #[test]
    fn test_count_in_window_2() {
        let nums = vec![0, 1, 2, 3, 6];
        assert_eq!(count_in_window(&nums, 1, 2), 4);
        assert_eq!(count_in_window(&nums, 2, 3), 2);
    }

    #[test]
    fn test_count_in_window_3() {
        let nums = vec![0, 1, 2, 3, 4, 5, 8];
        assert_eq!(count_in_window(&nums, 1, 2), 4);

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
        assert_eq!(count_arrangements("day_10_self_example_2.txt").unwrap(), 7)
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
        // Turns out I undercounted by 3.
        assert_eq!(count_arrangements("day_10_self_example_3.txt").unwrap(), 14);
    }
}


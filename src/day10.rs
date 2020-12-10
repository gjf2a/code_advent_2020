use advent_code_lib::all_lines;
use std::io;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (count1, count3) = count_jolt_jumps(filename)?;
    Ok((count1 * count3).to_string())
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

fn deletable(nums: &[usize], start: usize, end: usize) -> bool {
    start <= end && start > 0 && end < nums.len() - 1 && nums[end + 1] - nums[start - 1] <= 3
}

fn count_arrangements(filename: &str) -> io::Result<usize> {
    let nums = make_joltage_vec(filename)?;
    let mut permutations = 1;
    for i in 1..nums.len() - 1 {
        if deletable(&nums, i, i) {
            permutations *= 2;
            let mut j = i - 1;
            while deletable(&nums, j, j) {
                j -= 1;
            }
            j += 1;
            let mut subtractions = 0;
            while j < i && !deletable(&nums, j, i) {
                subtractions *= 2;
                subtractions += 1;
                j += 1;
            }
            permutations -= subtractions;
        }
    }
    Ok(permutations)
}

fn count_arrangements1(filename: &str) -> io::Result<usize> {
    let nums = make_joltage_vec(filename)?;
    Ok(count_arrangements_help(&nums))
}

fn count_arrangements_help(nums: &[usize]) -> usize {
    if nums.len() == 0 {
        1
    } else if deletable(&nums, 1, 1) {
        2 * count_arrangements_help(&nums[1..])
    } else {
        count_arrangements_help(&nums[1..])
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

        // Algorithm:
        // 1: Deletable; p: 2
        // 2: Deletable; p: 4
        // 3: Deletable; p: 8
        // 4: Deletable; p: 16 - 1 = 15
        assert_eq!(count_arrangements("day_10_self_example_3.txt").unwrap(), 11);
    }
}
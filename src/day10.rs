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

fn deletable(nums: &Vec<usize>, i: usize) -> bool {
    i > 0 && i < nums.len() - 1 && nums[i+1] - nums[i-1] <= 3
}

fn count_arrangements(filename: &str) -> io::Result<usize> {
    let nums = make_joltage_vec(filename)?;
    let mut permutations = 1;
    for i in 1..nums.len() - 1 {
        if deletable(&nums, i) {
            permutations *= 2;
        }
    }
    Ok(permutations)
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
}
use advent_code_lib::file2nums;
use std::io;

fn count_jolt_jumps(filename: &str) -> io::Result<(usize, usize)> {
    let mut nums = file2nums(filename)?;
    nums.sort();
    nums.insert(0, 0);
    nums.push(nums.last().unwrap() + 3);
    let mut count1 = 0;
    let mut count3 = 0;
    for i in 0..nums.len() - 1 {
        let diff = nums[i + 1] - nums[i];
        if diff == 1 {count1 += 1;}
        if diff == 3 {count3 += 1;}
    }
    Ok((count1, count3))
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
}
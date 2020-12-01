use std::io;
use crate::file2nums;

pub fn solve_1() -> io::Result<String> {
    let nums = file2nums("day_1_input.txt")?;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 {
                return Ok(format!("{} + {} == 2020; {} * {} == {}", nums[i], nums[j], nums[i], nums[j], nums[i] * nums[j]));
            }
        }
    }
    Ok("Failed".to_string())
}

pub fn solve_2() -> io::Result<String> {
    let nums = file2nums("day_1_input.txt")?;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            for k in j..nums.len() {
                if nums[i] + nums[j] + nums[k] == 2020 {
                    return Ok(format!("{} + {} + {} == 2020; {} * {} * {} == {}", nums[i], nums[j], nums[k], nums[i], nums[j], nums[k], nums[i] * nums[j] * nums[k]));
                }
            }
        }
    }
    Ok("Failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(solve_1().unwrap().as_str(), "1477 + 543 == 2020; 1477 * 543 == 802011")
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_2().unwrap().as_str(), "422 + 577 + 1021 == 2020; 422 * 577 * 1021 == 248607374")
    }

}
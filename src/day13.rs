use std::io;
use advent_code_lib::all_lines;
use num::Integer;
use bare_metal_modulo::ModNum;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (earliest_departure, busses) = puzzle_1_inputs(filename)?;
    let (best, wait) = best_bus_and_wait(&busses, earliest_departure);
    Ok((best * wait).to_string())
}

pub fn solve_2() -> io::Result<String> {
    let line_2 = all_lines("in/day13.txt")?.nth(1).unwrap();
    Ok(solve_2_str(line_2.as_str()).to_string())
}

fn puzzle_1_inputs(filename: &str) -> io::Result<(usize, Vec<usize>)> {
    let mut lines = all_lines(filename)?;
    let earliest_departure = lines.next().unwrap().parse::<usize>().unwrap();
    let busses = lines.next().unwrap().split(',')
        .filter(|n| *n != "x")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    Ok((earliest_departure, busses))
}

fn best_bus_and_wait(busses: &[usize], earliest_departure: usize) -> (usize, usize) {
    let (departure, best_bus) = busses.iter()
        .map(|bus| (bus_departure(*bus, earliest_departure), *bus))
        .min().unwrap();
    (best_bus, departure - earliest_departure)
}

fn bus_departure(bus: usize, earliest_departure: usize) -> usize {
    earliest_departure + bus - earliest_departure.mod_floor(&bus)
}

// Uses Extended Greatest Common Divisor algorithm
// I learned about all this from:
// https://byorgey.wordpress.com/2020/02/15/competitive-programming-in-haskell-modular-arithmetic-part-1/
// https://byorgey.wordpress.com/2020/03/03/competitive-programming-in-haskell-modular-arithmetic-part-2/
fn solve_2_str(input_line: &str) -> i128 {
    ModNum::chinese_remainder_system(
        &mut input_line
            .split(',')
            .enumerate()
            .filter(|(_, s)| *s != "x")
            .map(|(i, s)| -ModNum::new(i as i128, s.parse::<i128>().unwrap()))
            .inspect(|m| println!("{:?}", m)))
        .unwrap()
        .a()
}

/*
x = a (mod m)
x = b (mod n)
x = c (mod mn)
Imagine 7 and 13, separated by 1.
x = 0 (mod 7)
x = -1 (mod 13) = 12 (mod 13)
Find u and v so that 7u + 13v = 1: u = 2, v = -1
c = anv + bmu = 0 + 12 * 7 * 2 = 168
168 cong a mod 7
168 cong 12 mod 13
Note this also works if we do it this way:
x = 1 (mod 7)
x = 0 (mod 13)
u, v still 2, -1
c = 1 * 13 * -1 + 0 = -13
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_departure_1() {
        [(7, 945), (13, 949), (59, 944), (31, 961), (19, 950)].iter()
            .for_each(|(bus, depart)| assert_eq!(bus_departure(*bus, 939), *depart));
    }

    #[test]
    fn test_calculation_1() {
        let (bus, wait) = best_bus_and_wait(&[7, 13, 59, 31, 19], 939);
        assert_eq!(bus, 59);
        assert_eq!(wait, 5);
    }

    #[test]
    fn test_puzzle_1_inputs() {
        let (depart, busses) = puzzle_1_inputs("in/day13_ex.txt").unwrap();
        assert_eq!(depart, 939);
        assert_eq!(busses, vec![7,13,59,31,19]);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("in/day13_ex.txt").unwrap(), "295");
    }

    #[test]
    fn test_2_2() {
        for (line, goal) in &[
            ("17,x,13,19", 3417),
            ("67,7,59,61", 754018),
            ("67,x,7,59,61", 779210),
            ("67,7,x,59,61", 1261476),
            ("1789,37,47,1889", 1202161486)] {
            assert_eq!(solve_2_str(line), *goal);
        }
    }
}

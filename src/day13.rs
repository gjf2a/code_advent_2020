use std::io;
use advent_code_lib::all_lines;
use modulo::Mod;
use crate::modulo_article::egcd;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (earliest_departure, busses) = puzzle_1_inputs(filename)?;
    let (best, wait) = best_bus_and_wait(&busses, earliest_departure);
    Ok((best * wait).to_string())
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
    earliest_departure + bus - earliest_departure % bus
}

fn puzzle_2_inputs(filename: &str) -> io::Result<Vec<(i64,i64)>> {
    let line_2 = all_lines(filename)?.skip(1).next().unwrap();
    Ok(puzzle_2_line(line_2.as_str()))
}

// First value is the bus number; second value is the offset from the first bus
fn puzzle_2_line(line: &str) -> Vec<(i64,i64)> {
    let busses: Vec<&str> = line.split(',').collect();
    (0..busses.len())
        .filter(|i| busses[*i] != "x")
        .map(|i| (busses[i].parse::<i64>().unwrap(), i as i64))
        .collect()
}

// Uses Extended Greatest Common Divisor algorithm
// I learned about all this from:
// https://byorgey.wordpress.com/2020/02/15/competitive-programming-in-haskell-modular-arithmetic-part-1/
// https://byorgey.wordpress.com/2020/03/03/competitive-programming-in-haskell-modular-arithmetic-part-2/

fn puzzle_2_solver(p2line: &Vec<(i64,i64)>) -> (i64,i64) {
    p2line.iter()
        .map(|(m, a)| (*m, *a))
        .fold_first(|(m, a), (n, b)| {
        let (g, u, v) = egcd(m, n);
        (a.modulo(m) * n * v + b.modulo(n) * m * u, m * n)
    }).unwrap()
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

pub fn solve_2() -> io::Result<String> {
    let bus_offsets = puzzle_2_inputs("day13.txt")?;
    Ok(earliest_timestamp_for(&bus_offsets).to_string())
}

fn earliest_timestamp_for(bus_offsets: &[(i64,i64)]) -> i64 {
    let (max_bus, max_offset) = bus_offsets.iter().max().unwrap();
    let mut timestamp = *max_bus;
    while !timestamp_works(timestamp - *max_offset, bus_offsets) {timestamp += *max_bus;}
    timestamp - *max_offset
}
/*
fn earliest_timestamp_brute_force(bus_offsets: &[(usize, usize)]) -> usize {
    let interval = bus_offsets[0].0;
    let mut timestamp = interval;
    while !timestamp_works(timestamp, bus_offsets) {timestamp += interval;}
    timestamp
}
*/

fn timestamp_works(timestamp: i64, bus_offsets: &[(i64,i64)]) -> bool {
    bus_offsets.iter().all(|(bus, offset)| (timestamp + *offset) % *bus == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modulo_article::gcd;

    fn earliest_pair_timestamp(bus1: i64, bus2: i64, interval: i64) -> i64 {
        let mut timestamp = bus1;
        while !timestamp_works(timestamp, &[(bus1,0), (bus2,1)]) {timestamp += bus1;}
        timestamp * interval
    }

    fn earliest_pair_timestamps(bus_offsets: &[(i64, i64)]) -> Vec<i64> {
        let mut iter = bus_offsets.iter();
        let base_bus = iter.next().unwrap().0;
        iter.map(|(bus, offset)| earliest_pair_timestamp(base_bus, *bus, *offset))
            .collect()
    }

    fn lcm(a: i64, b: i64) -> i64 {
        (a * b) / gcd(a, b)
    }

    fn lcm_from(nums: &[i64]) -> i64 {
        nums.iter().fold(1, |acc, x| lcm(acc, *x))
    }

    #[test]
    fn puzzle_input_relatively_prime() {
        let inputs: Vec<_> = puzzle_2_inputs("in/day13.txt").unwrap().iter().map(|(n, _)| *n).collect();
        assert_eq!(inputs, vec![23, 41, 37, 421, 17, 19, 29, 487, 13]);
        for i in 0..inputs.len() {
            let ni = inputs[i];
            for j in i + 1..inputs.len() {
                let nj = inputs[j];
                assert_eq!(gcd(ni, nj), 1);
            }
        }
    }

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
    fn test_puzzle_2_inputs() {
        assert_eq!(puzzle_2_inputs("in/day13_ex.txt").unwrap(),
                   vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)]);
    }

    #[test]
    fn test_2_1() {
        assert_eq!(earliest_timestamp_for(&puzzle_2_inputs("in/day13_ex.txt").unwrap()), 1068781);
    }

    fn test_line(line: &str, goal: i64) {
        assert_eq!(earliest_timestamp_for(&puzzle_2_line(line)), goal);
        println!("goal: {}; new version: {:?}", goal, puzzle_2_solver(&puzzle_2_line(line)));
    }

    #[test]
    fn test_2_2() {
        for (line, goal) in &[("17,x,13,19", 3417), ("67,7,59,61", 754018), ("67,x,7,59,61", 779210), ("67,7,x,59,61", 1261476)] {
            test_line(line, *goal);
        }
    }

    #[test]
    fn test_2_3() {
        test_line("1789,37,47,1889", 1202161486);
    }

    #[test]
    fn idea() {
        for line in &["17,x,13,19", "67,7,59,61", "67,x,7,59,61", "67,7,x,59,61", "1789,37,47,1889"] {
            let nums = puzzle_2_line(line);
            let pair_times = earliest_pair_timestamps(&nums);
            let pair_steps: Vec<_> = pair_times.iter().map(|n| n / nums[0].0).collect();
            let brute = earliest_timestamp_for(&nums);
            println!("{}: pair times: {:?}: pair steps: {:?}, lcm: {} brute force: {} ({})", line, pair_times, pair_steps, lcm_from(&pair_times), brute, brute / nums[0].0);
        }
    }
}
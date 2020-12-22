use std::io;
use advent_code_lib::all_lines;
use num::bigint::BigInt;
use num::integer::mod_floor;
use num::abs;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (earliest_departure, busses) = puzzle_1_inputs(filename)?;
    let (best, wait) = best_bus_and_wait(&busses, earliest_departure);
    Ok((best * wait).to_string())
}

pub fn solve_2() -> io::Result<String> {
    let bus_offsets = puzzle_2_inputs("in/day13.txt")?;
    Ok(puzzle_2_solver(&bus_offsets).to_string())
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

fn puzzle_2_inputs(filename: &str) -> io::Result<Vec<(BigInt,BigInt)>> {
    let line_2 = all_lines(filename)?.skip(1).next().unwrap();
    Ok(puzzle_2_line(line_2.as_str()))
}

// First value is the bus number; second value is the offset from the first bus
fn puzzle_2_line(line: &str) -> Vec<(BigInt,BigInt)> {
    let busses: Vec<&str> = line.split(',').collect();
    (0..busses.len())
        .filter(|i| busses[*i] != "x")
        .map(|i| (busses[i].parse::<BigInt>().unwrap(), BigInt::from(i)))
        .collect()
}

// Uses Extended Greatest Common Divisor algorithm
// I learned about all this from:
// https://byorgey.wordpress.com/2020/02/15/competitive-programming-in-haskell-modular-arithmetic-part-1/
// https://byorgey.wordpress.com/2020/03/03/competitive-programming-in-haskell-modular-arithmetic-part-2/

fn puzzle_2_solver(p2line: &Vec<(BigInt, BigInt)>) -> BigInt {
    p2line.iter()
        .map(|(m, a)| (m.clone(), -a.clone()))
        .fold_first(|(m, a), (n, b)| {
            let (g, u, v) = egcd(&m, &n);
            let c = mod_floor((&a * &n * &v + &b * &m * &u) / &g, &m*&n);
            (&m * &n, c)
        }).unwrap().1.clone()
}

pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    if b == &num::zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

pub fn egcd(a: &BigInt, b: &BigInt) -> (BigInt,BigInt,BigInt) {
    if b == &num::zero() {
        (abs(a.clone()), if a < &num::zero() {-num::one::<BigInt>()} else {num::one()}, num::zero())
    } else {
        let (g, x, y) = egcd(b, &(a % b));
        (g, y.clone(), x - (a / b) * y)
    }
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
    fn puzzle_input_relatively_prime() {
        let inputs: Vec<_> = puzzle_2_inputs("in/day13.txt").unwrap().iter().map(|(n, _)| n.clone()).collect();
        let targets: Vec<_> = [23, 41, 37, 421, 17, 19, 29, 487, 13].iter().map(|i| BigInt::from(*i)).collect();
        assert_eq!(inputs, targets);
        for i in 0..inputs.len() {
            for j in i + 1..inputs.len() {
                assert_eq!(gcd(&inputs[i], &inputs[j]), num::one());
            }
        }
    }

    #[test]
    pub fn test_egcd() {
        for (a, b, g) in [(20, 12, 4), (25, 15, 5), (40, 35, 5), (220, 121, 11), (7, 13, 1)].iter().map(|(a,b,g)| (BigInt::from(*a), BigInt::from(*b), BigInt::from(*g))) {
            let (gp, x, y) = egcd(&a, &b);
            assert_eq!(gp, g);
            assert_eq!(&x * &a + &y * &b, g);
            println!("egcd({}, {}) = {} = {}*{} + {}*{}", a, b, gp, a, x, b, y);
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
                   [(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)].iter()
                       .map(|(a, b)| (BigInt::from(*a), BigInt::from(*b)))
                       .collect::<Vec<(BigInt,BigInt)>>());
    }

    fn test_line(line: &str, goal: BigInt) {
        assert_eq!(puzzle_2_solver(&puzzle_2_line(line)), goal);
    }

    #[test]
    fn test_2_2() {
        for (line, goal) in &[("17,x,13,19", 3417), ("67,7,59,61", 754018), ("67,x,7,59,61", 779210), ("67,7,x,59,61", 1261476)] {
            test_line(line, BigInt::from(*goal));
        }
    }

    #[test]
    fn test_2_3() {
        test_line("1789,37,47,1889", BigInt::from(1202161486));
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2().unwrap(), "667437230788118");
    }
}
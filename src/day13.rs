use std::io;
use advent_code_lib::all_lines;

pub fn solve_1(filename: &str) -> io::Result<String> {
    let (earliest_departure, busses) = puzzle_1_inputs(filename)?;
    let (best, wait) = best_bus_and_wait(&busses, earliest_departure);
    Ok((best * wait).to_string())
}

fn puzzle_1_inputs(filename: &str) -> io::Result<(usize, Vec<usize>)> {
    let mut lines = all_lines(filename)?;
    let earliest_departure = lines.next().unwrap()?.parse::<usize>().unwrap();
    let busses = lines.next().unwrap()?.split(',')
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

fn puzzle_2_inputs(filename: &str) -> io::Result<Vec<(usize,usize)>> {
    let line_2 = all_lines(filename)?.skip(1).next().unwrap()?;
    Ok(puzzle_2_line(line_2.as_str()))
}

fn puzzle_2_line(line: &str) -> Vec<(usize,usize)> {
    let busses: Vec<&str> = line.split(',').collect();
    (0..busses.len())
        .filter(|i| busses[*i] != "x")
        .map(|i| (busses[i].parse::<usize>().unwrap(), i))
        .collect()
}

fn earliest_timestamp_for(bus_offsets: &[(usize,usize)]) -> usize {
    earliest_timestamp_brute_for(bus_offsets)
}

fn earliest_timestamp_brute_for(bus_offsets: &[(usize,usize)]) -> usize {
    let mut timestamp = 1;
    while !timestamp_works(timestamp, bus_offsets) {timestamp += 1;}
    timestamp
}

fn timestamp_works(timestamp: usize, bus_offsets: &[(usize,usize)]) -> bool {
    bus_offsets.iter().all(|(bus, offset)| (timestamp + *offset) % *bus == 0)
}

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
        let (depart, busses) = puzzle_1_inputs("day_13_example.txt").unwrap();
        assert_eq!(depart, 939);
        assert_eq!(busses, vec![7,13,59,31,19]);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve_1("day_13_example.txt").unwrap(), "295");
    }

    #[test]
    fn test_puzzle_2_inputs() {
        assert_eq!(puzzle_2_inputs("day_13_example.txt").unwrap(),
                   vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)]);
    }

    #[test]
    fn test_2_1() {
        assert_eq!(earliest_timestamp_for(&puzzle_2_inputs("day_13_example.txt").unwrap()), 1068788);
    }

    fn test_line(line: &str, goal: usize) {
        assert_eq!(earliest_timestamp_for(&puzzle_2_line(line)), goal);
    }

    #[test]
    fn test_2_2() {
        test_line("17,x,13,19", 3417);
    }

    #[test]
    fn test_2_3() {
        for (line, goal) in &[("67,7,59,61", 754018), ("67,x,7,59,61", 779210), ("67,7,x,59,61", 1261476), ("1789,37,47,1889", 1202161486)] {
            test_line(line, *goal);
        }
    }
}
pub fn modular_exponentiation_1(b: u64, e: u32, m: u64) -> u64 {
    if e == 0 {
        1
    } else {
        let mut recursive = modular_exponentiation_1(b, e / 2, m).pow(2);
        if e % 2 == 1 {
            recursive *= b;
        }
        recursive % m
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn egcd(a: i64, b: i64) -> (i64,i64,i64) {
    if b == 0 {
        (a.abs(), if a < 0 {-1} else {1}, 0)
    } else {
        let (g, x, y) = egcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

pub fn modular_inverse(a: i64, m: i64) -> i64 {
    let (g, _, y) = egcd(m, a);
    assert_eq!(g, 1);
    ((y % m) + m) % m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_me1() {
        for (b, e, m) in &[
            (2, 10, 5), (3, 8, 7), (5, 5, 9)] {
            assert_eq!(modular_exponentiation_1(*b, *e, *m), b.pow(*e) % m);
        }
    }

    #[test]
    pub fn test_egcd() {
        for (a, b, g) in &[
            (20, 12, 4), (25, 15, 5), (40, 35, 5), (220, 121, 11)] {
            let (gp, x, y) = egcd(*a, *b);
            assert_eq!(gp, *g);
            assert_eq!(x * a + y * b, *g);
            println!("egcd({}, {}) = {} = {}*{} + {}*{}", a, b, gp, a, x, b, y);
        }
    }

    #[test]
    pub fn test_mod_inverse() {
        for (a, m) in &[
            (3, 7), (4, 13), (7, 10)] {
            let b = modular_inverse(*a, *m);
            println!("Inverse of {} (mod {}) = {}", a, m, b);
            assert_eq!(a * b % m, 1);
        }
    }
}
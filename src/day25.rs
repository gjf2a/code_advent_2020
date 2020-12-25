use num::bigint::BigInt;
use std::mem;
use num::Integer;

pub fn solve_1() -> String {
    find_encryption_key(BigInt::from(19241437), BigInt::from(17346587)).to_string()
}

fn find_encryption_key(card_public: BigInt, door_public: BigInt) -> BigInt {
    let card_loop = find_loop_size(&card_public);
    let door_loop = find_loop_size(&door_public);
    let guess1 = guess_encryption_key(card_loop, &door_public);
    let guess2 = guess_encryption_key(door_loop, &card_public);
    assert_eq!(guess1, guess2);
    guess1
}

fn find_loop_size(public_key: &BigInt) -> usize {
    let public_subject = BigInt::from(7);

    Transform::from(public_subject.clone())
        .enumerate()
        .find(|(_, n)| n == public_key)
        .map(|(c,_)| c)
        .unwrap()
}

fn guess_encryption_key(device_1_loop: usize, device_2_public_key: &BigInt) -> BigInt {
    Transform::from(device_2_public_key.clone()).nth(device_1_loop).unwrap()
}

#[derive(Debug,Clone)]
struct Transform {
    subject_number: BigInt,
    value: BigInt
}

impl Transform {
    fn from(subject_number: BigInt) -> Self {
        Transform {subject_number, value: BigInt::from(1)}
    }
}

impl Iterator for Transform {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let loop_mod = BigInt::from(20_201_227);
        let mut future = (&self.value * &self.subject_number).mod_floor(&loop_mod);
        mem::swap(&mut future, &mut self.value);
        Some(future)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_size() {
        assert_eq!(find_loop_size(&BigInt::from(5764801)), 8);
        assert_eq!(find_loop_size(&BigInt::from(17807724)), 11);
    }

    #[test]
    fn test_find_key() {
        assert_eq!(find_encryption_key(BigInt::from(17807724), BigInt::from(5764801)), BigInt::from(14897079));
    }
}
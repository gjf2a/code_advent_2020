use std::mem;
use num::Integer;

const PUBLIC_SUBJECT: i128 = 7;
const LOOP_MOD: i128 = 20_201_227;

pub fn solve_1() -> String {
    find_encryption_key(19241437, 17346587).to_string()
}

fn find_encryption_key(card_public: i128, door_public: i128) -> i128 {
    let card_loop = find_loop_size(card_public);
    let door_loop = find_loop_size(door_public);
    let guess1 = guess_encryption_key(card_loop, &door_public);
    let guess2 = guess_encryption_key(door_loop, &card_public);
    assert_eq!(guess1, guess2);
    guess1
}

fn find_loop_size(public_key: i128) -> usize {
    Transform::from(PUBLIC_SUBJECT)
        .enumerate()
        .find(|(_, n)| n == &public_key)
        .map(|(c,_)| c)
        .unwrap()
}

fn guess_encryption_key(device_1_loop: usize, device_2_public_key: &i128) -> i128 {
    Transform::from(device_2_public_key.clone()).nth(device_1_loop).unwrap()
}

#[derive(Debug,Clone)]
struct Transform {
    subject_number: i128,
    value: i128
}

impl Transform {
    fn from(subject_number: i128) -> Self {
        Transform {subject_number, value: 1}
    }
}

impl Iterator for Transform {
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        let mut future = (&self.value * &self.subject_number).mod_floor(&LOOP_MOD);
        mem::swap(&mut future, &mut self.value);
        Some(future)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_size() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
    }

    #[test]
    fn test_find_key() {
        assert_eq!(find_encryption_key(17807724, 5764801), 14897079);
    }
}
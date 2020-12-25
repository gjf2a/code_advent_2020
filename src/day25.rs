use num::bigint::BigInt;
use std::mem;
use num::Integer;

fn find_encryption_key(card_public: BigInt, door_public: BigInt) -> BigInt {
    let card_loop = find_loop_size(&card_public);
    let door_loop = find_loop_size(&door_public);
    card_loop // placeholder
}

fn find_loop_size(public_key: &BigInt) -> BigInt {
    let public_subject = BigInt::from(7);

    BigInt::from(Transform::from(public_subject.clone())
        .enumerate()
        .find(|(c, n)| n == public_key)
        .unwrap().0)
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
        assert_eq!(find_loop_size(&BigInt::from(5764801)), BigInt::from(8));
        assert_eq!(find_loop_size(&BigInt::from(17807724)), BigInt::from(11));
    }
}
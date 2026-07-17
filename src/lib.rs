use aws_lc_rs::error::Unspecified;

#[unsafe(no_mangle)]
pub fn add(left: u64, right: u64) -> u64 {
    let e = Unspecified;

    left + right + format!("{:?}", e).len() as u64
}

pub fn foo() {
    let rng = aws_lc_rs::rand::SystemRandom::new();
    let buf: [u8; 4] = aws_lc_rs::rand::generate(&rng).unwrap().expose();
    dbg!(buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

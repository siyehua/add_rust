
/// plus value + 1 and return
pub fn add(left: i32) -> i32 {
    left + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2);
        assert_eq!(result, 3);
    }
}

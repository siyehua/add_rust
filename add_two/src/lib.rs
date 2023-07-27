
/// plus value + 2 and return
pub fn add(left: i32) -> i32 {
    left + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2);
        assert_eq!(result, 4);
    }
}

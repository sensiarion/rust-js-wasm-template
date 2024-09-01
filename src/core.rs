pub fn add(left: usize, right: usize) -> String {
    (left + right).to_string()
}

#[cfg(test)]
mod tests {
    use crate::core::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, "4");
    }
}
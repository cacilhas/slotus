#[macro_export]
macro_rules! s {
    ($s:literal) => {
        $s.to_string()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_string_from_str() {
        assert_eq!(s!["test"], "test".to_string());
    }

    #[test]
    fn it_should_return_a_string_from_i64() {
        assert_eq!(s![1], "1".to_string());
    }

    #[test]
    fn it_should_return_a_string_from_f64() {
        assert_eq!(s![1.5], "1.5".to_string());
    }
}

use mzbatch_generator::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_float_value() {
        let value = Some(3.14);
        let val = Value::Float(&value);
        assert_eq!(val.get_float_value(), &Some(3.14));
    }

    #[test]
    #[should_panic(expected = "Not &Option<f32> returning parameter")]
    fn get_float_value_panic() {
        let val = Value::Str("test");
        val.get_float_value(); // should panic
    }

    #[test]
    fn get_str_value() {
        let val = Value::Str("test");
        assert_eq!(val.get_str_value(), "test");
    }

    #[test]
    #[should_panic(expected = "Not a &str returning parameter")]
    fn get_str_value_panic() {
        let value = Some(3.14);
        let val = Value::Float(&value);
        val.get_str_value(); // should panic
    }
}
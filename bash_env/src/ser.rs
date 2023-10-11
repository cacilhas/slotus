use json::JsonValue;

pub fn stringify(value: JsonValue) -> String {
    match value {
        JsonValue::Null => "".to_string(),
        JsonValue::Boolean(true) => "1".to_string(),
        JsonValue::Boolean(false) => "".to_string(),

        JsonValue::Number(value) if value.is_nan() => "".to_string(),
        JsonValue::Number(value) if value.is_zero() || value.is_empty() => "0".to_string(),
        JsonValue::Number(value) => value.to_string(),

        JsonValue::Short(value) => json::stringify(value.to_string()),
        JsonValue::String(value) => json::stringify(value),

        JsonValue::Array(value) => {
            "(".to_string()
                + &value
                    .iter()
                    .map(|v| stringify(v.to_owned()))
                    .collect::<Vec<String>>()
                    .join(" ")
                + ")"
        }

        JsonValue::Object(value) => {
            "(".to_string()
                + &value
                    .iter()
                    .map(|(key, value)| "[".to_string() + key + "]=" + &stringify(value.to_owned()))
                    .collect::<Vec<String>>()
                    .join(" ")
                + ")"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_serialise_null() {
        assert!(stringify(JsonValue::Null).is_empty());
    }

    #[test]
    fn it_should_serialise_bool() {
        assert_eq!(stringify(true.into()), "1".to_string());
        assert!(stringify(false.into()).is_empty());
    }

    #[test]
    fn it_should_serialise_number() {
        assert_eq!(stringify(0.into()), "0".to_string());
        assert_eq!(stringify(1.into()), "1".to_string());
        assert_eq!(stringify(1.5.into()), "1.5".to_string());
        assert!(stringify(std::f64::NAN.into()).is_empty());
    }

    #[test]
    fn it_should_serialise_string() {
        assert_eq!(stringify("".into()), r#""""#.to_string());
        assert_eq!(stringify("test".into()), r#""test""#.to_string());
        assert_eq!(stringify("test\"it".into()), r#""test\"it""#.to_string());
        assert_eq!(stringify("(a b c)".into()), r#""(a b c)""#.to_string());
    }

    #[test]
    fn it_should_serialise_array() {
        let value = json::parse(r#"["name", "x", 12, ["test", true]]"#).unwrap();
        assert_eq!(
            stringify(value),
            r#"("name" "x" 12 ("test" 1))"#.to_string(),
        );
    }

    #[test]
    fn it_should_serialise_object() {
        let value =
            json::parse(r#"{"name": "test", "x": 42, "ar": [1, 2, 3], "other": {"x": 3, "y": 4}}"#)
                .unwrap();
        assert_eq!(
            stringify(value),
            r#"([name]="test" [x]=42 [ar]=(1 2 3) [other]=([x]=3 [y]=4))"#.to_string(),
        );
    }

    #[test]
    fn empty_array() {
        let value: JsonValue = Vec::<JsonValue>::new().into();
        assert_eq!(stringify(value), "()".to_string());
    }
}

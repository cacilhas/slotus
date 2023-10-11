use json::JsonValue;

pub fn stringify(value: JsonValue) -> String {
    match value {
        JsonValue::Null => String::default(),
        JsonValue::Boolean(true) => s![1],
        JsonValue::Boolean(false) => String::default(),

        JsonValue::Number(value) if value.is_nan() => String::default(),
        JsonValue::Number(value) if value.is_zero() || value.is_empty() => s![0],
        JsonValue::Number(value) => value.to_string(),

        JsonValue::Short(value) => json::stringify(value.to_string()),
        JsonValue::String(value) => json::stringify(value),

        JsonValue::Array(value) => {
            s!["("]
                + &value
                    .iter()
                    .map(|v| stringify(v.to_owned()))
                    .collect::<Vec<String>>()
                    .join(" ")
                + ")"
        }

        JsonValue::Object(value) => {
            s!["("]
                + &value
                    .iter()
                    .map(|(key, value)| s!["["] + key + "]=" + &stringify(value.to_owned()))
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
        assert_eq!(stringify(true.into()), s![1]);
        assert!(stringify(false.into()).is_empty());
    }

    #[test]
    fn it_should_serialise_number() {
        assert_eq!(stringify(0.into()), s![0]);
        assert_eq!(stringify(1.into()), s![1]);
        assert_eq!(stringify(1.5.into()), s![1.5]);
        assert!(stringify(std::f64::NAN.into()).is_empty());
    }

    #[test]
    fn it_should_serialise_string() {
        assert_eq!(stringify("".into()), s![r#""""#]);
        assert_eq!(stringify("test".into()), s![r#""test""#]);
        assert_eq!(stringify("test\"it".into()), s![r#""test\"it""#]);
        assert_eq!(stringify("(a b c)".into()), s![r#""(a b c)""#]);
    }

    #[test]
    fn it_should_serialise_array() {
        let value = json::parse(r#"["name", "x", 12, ["test", true]]"#).unwrap();
        assert_eq!(stringify(value), s![r#"("name" "x" 12 ("test" 1))"#]);
    }

    #[test]
    fn it_should_serialise_object() {
        let value =
            json::parse(r#"{"name": "test", "x": 42, "ar": [1, 2, 3], "other": {"x": 3, "y": 4}}"#)
                .unwrap();
        assert_eq!(
            stringify(value),
            s![r#"([name]="test" [x]=42 [ar]=(1 2 3) [other]=([x]=3 [y]=4))"#],
        );
    }

    #[test]
    fn empty_array() {
        let value: JsonValue = Vec::<JsonValue>::new().into();
        assert_eq!(stringify(value), s!["()"]);
    }
}

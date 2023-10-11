#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

use json::*;
use pest::{iterators::Pair, Parser};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Ord, PartialOrd, Parser)]
#[grammar = "assets/bash.pest"]
struct Bash;

pub fn parse(value: &str) -> eyre::Result<JsonValue> {
    for pair in Bash::parse(Rule::main, &value)? {
        return parser_to_json(pair);
    }
    Err(eyre::eyre!("no main parsed from: {}", value))
}

fn parser_to_json(pair: Pair<Rule>) -> eyre::Result<JsonValue> {
    match pair.as_rule() {
        Rule::main => {
            let first = pair.to_owned().into_inner().next();
            match first {
                Some(inner) if inner.as_rule() == Rule::attrib => {
                    let mut res: HashMap<String, JsonValue> = HashMap::new();
                    for inner in pair.to_owned().into_inner() {
                        let mut key = String::default();
                        for inner in inner.to_owned().into_inner() {
                            match inner.as_rule() {
                                Rule::id => {
                                    key = inner.as_str().to_string();
                                }
                                Rule::value => {
                                    res.insert(key.to_owned(), parser_to_json(inner)?);
                                }
                                _ => (),
                            }
                        }
                    }
                    return Ok(res.into());
                }
                Some(inner) if inner.as_rule() == Rule::value => parser_to_json(inner),
                _ => Err(eyre::eyre!("unexpected value: {}", pair.as_str())),
            }
        }

        Rule::value => match pair.to_owned().into_inner().next() {
            Some(value) => parser_to_json(value),
            None => Err(eyre::eyre!("unexpected empty value: {}", pair)),
        },

        Rule::id => Ok(pair.as_str().into()),
        Rule::number => {
            let s = pair.as_str();
            match s.parse::<f64>() {
                Ok(value) => Ok(value.into()),
                Err(err) => Err(eyre::eyre!("unexpected number error: {}", err)),
            }
        }
        Rule::string => {
            let mut res = String::default();
            for part in pair.into_inner() {
                res = res + stringify_token(part)?;
            }
            Ok(res.into())
        }
        Rule::array => {
            let mut res: Vec<JsonValue> = Vec::new();
            for element in pair.into_inner() {
                match element.as_rule() {
                    Rule::value => {
                        res.push(parser_to_json(element)?);
                    }
                    _ => (),
                }
            }
            Ok(res.into())
        }
        Rule::hash => {
            let mut res: HashMap<String, JsonValue> = HashMap::new();
            for part in pair.into_inner() {
                match part.as_rule() {
                    Rule::pair => {
                        let mut key = String::default();
                        for kv in part.into_inner() {
                            match kv.as_rule() {
                                Rule::id => {
                                    key = kv.as_str().to_string();
                                }
                                Rule::value => {
                                    res.insert(key.to_owned(), parser_to_json(kv)?);
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
            Ok(res.into())
        }

        _ => Err(eyre::eyre!("unexpected token: {}", pair)),
    }
}

fn stringify_token<'a>(pair: Pair<'a, Rule>) -> eyre::Result<&'a str> {
    match pair.as_rule() {
        Rule::str_space => Ok(" "),
        Rule::str_dbl_quote => Ok("\""),
        Rule::str_quote => Ok("'"),
        Rule::alphanum | Rule::no_dbl_quote | Rule::no_quote => Ok(pair.as_str()),
        Rule::dbl_quote | Rule::quote => Ok(""),

        _ => Err(eyre::eyre!("unexpected token to stringify: {}", pair)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // let _ = parse("RES=([test]=(abc def \"ghi jkl\" aj\\ 9 12 1.0))".to_string());
    #[test]
    fn it_should_parse_simple_string() {
        let expected: JsonValue = "some_test123".into();
        assert_eq!(parse(r"some_test123").unwrap(), expected);
    }

    #[test]
    fn it_should_parse_string_with_space() {
        let expected: JsonValue = "some test123".into();
        assert_eq!(parse(r"some\ test123").unwrap(), expected);
    }

    #[test]
    fn it_should_parse_string_with_quote() {
        let expected: JsonValue = "some'test123".into();
        assert_eq!(parse(r"some\'test123").unwrap(), expected);
    }

    #[test]
    fn it_should_parse_string_with_quotes() {
        let expected: JsonValue = "some test 123".into();
        assert_eq!(parse(r"'some test 123'").unwrap(), expected);
    }

    #[test]
    fn it_should_parse_string_with_dbl_quote() {
        let expected: JsonValue = "some\"test123".into();
        assert_eq!(parse(r#"some\"test123"#).unwrap(), expected);
    }

    #[test]
    fn it_should_parse_string_with_dbl_quotes() {
        let expected: JsonValue = "some test'123".into();
        assert_eq!(parse(r#""some test'123""#).unwrap(), expected);
    }

    #[test]
    fn it_should_parse_array() {
        let expected = json::parse(r#"[12, "test", "a b", ["x", "y", "z"]]"#).unwrap();
        assert_eq!(parse(r#"(12 test 'a b' (x y z))"#).unwrap(), expected);
    }

    #[test]
    fn it_should_parse_object() {
        let expected =
            json::parse(r#"{"name": "test", "x": 3, "y": 4, "data": ["test'ing", "another"]}"#)
                .unwrap();
        assert_eq!(
            parse(r#"([name]=test [x]=3 [y]=4 [data]=(test\'ing 'another'))"#).unwrap(),
            expected,
        );
    }

    #[test]
    fn it_should_parse_object_with_empty_value() {
        let expected = json::parse(r#"{"a": "test", "c": "[d]=e"}"#).unwrap();
        assert_eq!(parse(r#"([a]=test [b]= [c]="[d]=e")"#).unwrap(), expected,);
    }

    #[test]
    fn it_shoud_set_variables() {
        let expected =
            json::parse(r#"{"name": "test", "x": 3, "y": 4, "data": ["test'ing", "another"]}"#)
                .unwrap();
        assert_eq!(
            parse(r#"name=test x=3 y=4 data=(test\'ing 'another')"#).unwrap(),
            expected,
        )
    }

    #[test]
    fn it_should_deal_with_corner_cases() {
        let expected = json::parse(r#"["abc", "(def ghi)", "jkl"]"#).unwrap();
        assert_eq!(parse(r#"(abc '(def ghi)' jkl)"#).unwrap(), expected,);
    }
}

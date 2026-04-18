use crate::utils::error::JsonError;
use crate::utils::token::Token;
use std::collections::BTreeMap as Map;

#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Json>),
    Object(Map<String, Json>),
}

#[derive(Debug, PartialEq)]
pub enum JsonGet<'a> {
    Index(usize),
    Key(&'a str),
}

impl Json {
    pub fn parse(input: &str) -> Result<Json, JsonError> {
        use JsonError::*;

        let tokens = Token::tokenize(input)?;
        Token::validate(&tokens)?;
        let mut tokens = tokens
            .into_iter()
            .filter(|token| *token != Token::Comma && *token != Token::Colon);
        match tokens.next() {
            Some(Token::ArrayOpen) => Self::parse_array(&mut tokens),
            Some(Token::ObjectOpen) => Self::parse_object(&mut tokens),
            // Shouldn't be reachable after validation
            _ => Err(InvalidToken((None, "wrong start"))),
        }
    }

    fn parse_value(value: &str) -> Result<Self, JsonError> {
        use Json::*;

        if value == "null" {
            Ok(Null)
        } else if let Ok(boolean) = value.parse::<bool>() {
            Ok(Bool(boolean))
        } else if let Ok(integer) = value.parse::<i64>() {
            Ok(Integer(integer))
        } else if let Ok(float) = value.parse::<f64>() {
            Ok(Float(float))
        } else {
            Err(JsonError::InvalidValue(value.to_string()))
        }
    }

    fn parse_array<'a, I>(tokens: &mut I) -> Result<Json, JsonError>
    where
        I: Iterator<Item = Token<'a>>    
    {
        use JsonError::*;
        use Token::*;

        let mut array = Vec::new();
        while let Some(token) = tokens.next() {
            if token == ArrayClose {
                return Ok(Self::Array(array));
            }
            let item = match token {
                String(string) => Self::String(string.to_string()),
                Value(value) => Self::parse_value(value)?,
                ArrayOpen => Self::parse_array(tokens)?,
                ObjectOpen => Self::parse_object(tokens)?,
                // Shouldn't be reachable after validation
                _ => return Err(InvalidToken((None, "invalid item in array"))),
            };
            array.push(item);
        }

        // Shouldn't be reachable after validation
        Err(InvalidToken((None, "array not closed")))
    }

    fn parse_object<'a, I>(tokens: &mut I) -> Result<Json, JsonError>
    where
        I: Iterator<Item = Token<'a>>    
    {
        use JsonError::*;
        use Token::*;

        let mut object = Map::new();
        while let Some(token) = tokens.next() {
            if token == ObjectClose {
                return Ok(Self::Object(object))
            }
            if let String(key) = token {
                if let Some(token) = tokens.next() {
                    let value = match token {
                        String(string) => Self::String(string.to_string()),
                        Value(value) => Self::parse_value(value)?,
                        ArrayOpen => Self::parse_array(tokens)?,
                        ObjectOpen => Self::parse_object(tokens)?,
                        // Shouldn't be reachable after validation
                        _ => return Err(InvalidToken((None, "invalid value in object"))),
                    };
                    object.insert(key.to_string(), value);
                } else {
                    break;
                }
            } else {
                // Shouldn't be reachable after validation
                return Err(InvalidToken((None, "invalid key in object (not a string)")));
            }
        }

        // Shouldn't be reachable after validation
        Err(InvalidToken((None, "object not closed")))
    }

    pub fn get(&self, key: JsonGet) -> Option<&Json> {
        match (self, key) {
            (Self::Array(array), JsonGet::Index(i)) => array.get(i),
            (Self::Object(map), JsonGet::Key(key)) => map.get(key),
            _ => None
        }
    }
}

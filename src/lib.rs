mod utils;

pub use crate::utils::error::JsonError;
pub use crate::utils::json::{Json, JsonGet};
use crate::utils::token::Token;

pub fn parse(input: &str) -> Result<Json, JsonError> {
    Json::parse(input)
}

pub fn validate(input: &str) -> Result<(), JsonError> {
    let tokens = Token::tokenize(input)?;
    Token::validate(&tokens)
}

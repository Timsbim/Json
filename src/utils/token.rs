use crate::utils::error::JsonError;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Comma,
    Colon,
    String(&'a str),
    Value(&'a str),
    ArrayOpen,
    ArrayClose,
    ObjectOpen,
    ObjectClose,
}

#[derive(Debug, PartialEq)]
enum State {
    In,
    Out,
    Array(u8),
    Object(u8),
}

impl<'a> Token<'a> {
    pub fn tokenize(input: &'a str) -> Result<Vec<Self>, JsonError> {
        use JsonError::*;
        use Token::*;

        let mut tokens = Vec::new();
        let mut token;
        // For extracting strings and values (null, bool, float, integer)
        let mut last_ch = ' ';
        let mut start = 0;
        let mut in_str = false;
        let mut in_val = false;
        for (i, ch) in input.char_indices() {
            if in_str {
                if ch == '"' && last_ch != '\\' {
                    tokens.push(String(&input[start..i]));
                    in_str = false;
                } else {
                    last_ch = ch;
                }
            } else if ch == '"' {
                if in_val {
                    return Err(InvalidString(Some(i)));
                }
                (in_str, last_ch, start) = (true, '"', i + 1);
            } else if ch.is_whitespace() {
                if in_val {
                    tokens.push(Value(&input[start..i]));
                    in_val = false;
                }
            } else {
                token = match ch {
                    ',' => Comma,
                    ':' => Colon,
                    '[' => ArrayOpen,
                    ']' => ArrayClose,
                    '{' => ObjectOpen,
                    '}' => ObjectClose,
                    _ => {
                        if !in_val {
                            (in_val, start) = (true, i);
                        }
                        continue;
                    }
                };
                if in_val {
                    tokens.push(Value(&input[start..i]));
                    in_val = false;
                }
                tokens.push(token);
            }
        }

        // Early validity check
        if in_str || in_val {
            Err(InvalidString(None))
        } else {
            Ok(tokens)
        }
    }

    pub fn validate(tokens: &[Token]) -> Result<(), JsonError> {
        use JsonError::*;
        use State::*;
        use Token::*;

        let mut state = In;
        let mut stack: Vec<State> = Vec::new();
        let mut last_token = &Comma;
        let mut p = Some(1);
        for token in tokens {
            state = match *token {
                Comma => match state {
                    Array(1) => Array(0),
                    Object(3) => Object(0),
                    _ => return Err(InvalidToken((p, ","))),
                },
                Colon => match state {
                    Object(1) => Object(2),
                    _ => return Err(InvalidToken((p, ":"))),
                },
                String(_) => match state {
                    Array(0) => Array(1),
                    Object(i) if i == 0 || i == 2 => Object(i + 1),
                    _ => return Err(InvalidToken((p, "string"))),
                },
                Value(_) => match state {
                    Array(0) => Array(1),
                    Object(2) => Object(3),
                    _ => return Err(InvalidToken((p, "value"))),
                },
                ArrayOpen => match state {
                    Out | Array(1) => return Err(InvalidToken((p, "["))),
                    Object(i) if i != 2 => return Err(InvalidToken((p, "["))),
                    _ => {
                        stack.push(state);
                        Array(0)
                    }
                },
                ArrayClose => match state {
                    Array(_) if *last_token != Comma => match stack.pop() {
                        Some(Array(_)) => Array(1),
                        Some(Object(_)) => Object(3),
                        _ => Out,
                    },
                    _ => return Err(InvalidToken((p, "]"))),
                },
                ObjectOpen => match state {
                    Out | Array(1) => return Err(InvalidToken((p, "{"))),
                    Object(i) if i != 2 => return Err(InvalidToken((p, "{"))),
                    _ => {
                        stack.push(state);
                        Object(0)
                    }
                },
                ObjectClose => match state {
                    Object(_) if *last_token != Comma => match stack.pop() {
                        Some(Array(_)) => Array(1),
                        Some(Object(_)) => Object(3),
                        _ => State::Out,
                    },
                    _ => return Err(InvalidToken((p, "}"))),
                },
            };

            last_token = token;
            p = p.map(|i| i + 1);
        }

        if state == State::Out {
            Ok(())
        } else {
            Err(InvalidToken((None, "object or array not closed")))
        }
    }
}

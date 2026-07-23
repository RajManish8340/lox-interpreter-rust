use crate::token::Token;

pub(crate) struct Parser {
    source: Box<Vec<Token>>,
}

impl Parser {
    pub(crate) fn new(tokens: Box<Vec<Token>>) -> Self {
        Self { source: tokens }
    }
}

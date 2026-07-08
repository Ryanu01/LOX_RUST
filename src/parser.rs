use crate::{
    ast::{BinaryOperator, Expression, LiteralValue, UnaryOperator},
    scanner::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

type ParseError = String;
type ParseResult<T> = Result<T, ParseError>;

pub fn parse_tokens(tokens: Vec<Token>) -> ParseResult<Expression> {
    let mut parser = Parser::new(tokens);
    parser.expression()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> ParseResult<Expression> {
        return self.equality();
    }

    fn equality(&mut self) -> ParseResult<Expression> {
        let mut expr = self.comparison()?;
        loop {
            let binary_operator = match self.peek().token_type {
                TokenType::BangEqual => BinaryOperator::BangEqual,
                TokenType::EqualEqual => BinaryOperator::EqualEqual,
                _ => break,
            };

            self.advance();
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> ParseResult<Expression> {
        let mut expr = self.term()?;
        loop {
            let binary_operator = match self.peek().token_type {
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                _ => break,
            };

            self.advance();
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn term(&mut self) -> ParseResult<Expression> {
        let mut expr = self.factor()?;
        loop {
            let binary_operator = match self.peek().token_type {
                TokenType::Minus => BinaryOperator::Minus,
                TokenType::Plus => BinaryOperator::Plus,
                _ => break,
            };

            self.advance();
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> ParseResult<Expression> {
        let mut expr = self.unary()?;
        loop {
            let binary_operator = match self.peek().token_type {
                TokenType::Slash => BinaryOperator::Slash,
                TokenType::Star => BinaryOperator::Star,
                _ => break,
            };

            self.advance();
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: binary_operator,
                right: Box::new(right),
            }
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> ParseResult<Expression> {
        let unary_operator = match self.peek().token_type {
            TokenType::Bang => UnaryOperator::Bang,
            TokenType::Minus => UnaryOperator::Minus,
            _ => return Ok(self.primary()?),
        };

        self.advance();
        let expr = self.unary()?;
        return Ok(Expression::Unary {
            operator: unary_operator,
            expression: Box::new(expr),
        });
    }

    fn primary(&mut self) -> ParseResult<Expression> {
        let literal = match &self.peek().token_type {
            TokenType::False => LiteralValue::Boolean(false),
            TokenType::True => LiteralValue::Boolean(true),
            TokenType::Nil => LiteralValue::Nil,
            TokenType::Number(num) => LiteralValue::Number(*num),
            TokenType::String(str) => LiteralValue::String(str.clone()),
            _ => return Err(format!("Unexpected token: {}", self.peek().lexeme)),
        };
        self.advance();
        return Ok(Expression::Literal { value: literal });
    }

    fn grouping(&mut self) -> ParseResult<Expression> {
        if matches!(self.peek().token_type, TokenType::LeftParen) {
            self.advance();
            let expr = self.expression()?;

            if matches!(self.peek().token_type, TokenType::RightParen) {
                self.advance();
                return Ok(Expression::Grouping {
                    expression: Box::new(expr),
                });
            } else {
                return Err(format!("Expected ')' got: {}", self.peek().lexeme));
            }
        } else {
            return Err(format!("Unexpected token: {}", self.peek().lexeme));
        }
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::Eof => true,
            _ => false,
        }
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }
}

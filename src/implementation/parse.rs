use crate::{
    tools::{
        parse::{
            Parsable,
            ParseResult,
            Parser
        },
        lex::Lexable
    },
    structure::{
        tokens::Token,
        position::{
            Position,
            Located
        },
        ast::*
    }
};

pub fn parse<T: Lexable, N: Parsable<T>>(tokens: Vec<Located<T>>) -> ParseResult<N> {
    N::parse(&mut Parser::new(tokens))
}

impl Atom {
    pub fn ident(parser: &mut Parser<Token>) -> ParseResult<String> {
        let Located { value: token, pos } = parser.expect()?;
        if let Token::Ident(ident) = token {
            Ok(Located::new(ident, pos))
        } else {
            Err(Located::new(format!("expected ident token, got token {token:?}"), pos))
        }
    }
    pub fn path(parser: &mut Parser<Token>) -> ParseResult<Self> {
        let mut head = Self::parse(parser)?;
        while let Some(Located { value: token, pos }) = parser.peek() {
            match token {
                Token::Field => {
                    parser.expect()?;
                    let field = Self::ident(parser)?;
                    let pos = Position::between(&head.pos, &field.pos);
                    head = Located::new(Self::Field {
                        head: Box::new(head),
                        field
                    }, pos)
                }
                Token::ArrayIn => {
                    parser.expect()?;
                    let field = Expression::parse(parser)?;
                    let pos = Position::between(&head.pos, &field.pos);
                    head = Located::new(Self::Index {
                        head: Box::new(head),
                        field: Box::new(field)
                    }, pos)
                }
                _ => break
            }
        }
        Ok(head)
    }
}
impl Parsable<Token> for Atom {
    fn parse(parser: &mut Parser<Token>) -> ParseResult<Self> {
        let Located { value: token, mut pos } = parser.expect()?;
        match token {
            Token::Int(v) => Ok(Located::new(Self::Int(v), pos)),
            Token::Float(v) => Ok(Located::new(Self::Float(v), pos)),
            Token::Boolean(v) => Ok(Located::new(Self::Boolean(v), pos)),
            Token::None => Ok(Located::new(Self::None, pos)),
            Token::String(v) => Ok(Located::new(Self::String(v), pos)),
            Token::Ident(v) => Ok(Located::new(Self::Ident(v), pos)),
            Token::ExprIn => {
                let expr = Expression::parse(parser)?;
                parser.expect_token(Token::ExprOut)?;
                Ok(Located::new(Self::Expression(Box::new(expr)), pos))
            }
            Token::ArrayIn => {
                let mut values = vec![];
                if let Some(Located { value: Token::ArrayOut, pos: _ }) = parser.peek() {
                    let Located { value: _, pos: end_pos } = parser.expect()?;
                    pos.extend(&end_pos);
                    return Ok(Located::new(Self::Vector(values), pos));
                }
                while let Some(Located { value: token, pos: _ }) = parser.peek() {
                    let arg = Expression::parse(parser)?;
                    values.push(arg);
                    if let Some(Located { value: Token::ArrayOut, pos: _ }) = parser.peek() {
                        let Located { value: _, pos: end_pos } = parser.expect()?;
                        pos.extend(&end_pos);
                        return Ok(Located::new(Self::Vector(values), pos));
                    }
                    if let Some(Located { value: token, pos }) = parser.get() {
                        if token != Token::Seperate {
                            return Err(Located::new(format!("expected token {:?} or {:?}, got token {token:?}", Token::Seperate, Token::ExprOut), pos))
                        }
                    }
                }
                Err(Located::new("unclosed vector".into(), pos))
            }
            token => Err(Located::new(format!("unexpected token: {token:?}"), pos))
        }
    }
}

impl Parsable<Token> for Args {
    fn parse(parser: &mut Parser<Token>) -> ParseResult<Self> {
        let Located { value: _, mut pos } = parser.expect_token(Token::ExprIn)?;
        let mut args = vec![];
        if let Some(Located { value: Token::ExprOut, pos: _ }) = parser.peek() {
            let Located { value: _, pos: end_pos } = parser.expect()?;
            pos.extend(&end_pos);
            return Ok(Located::new(Self(args), pos));
        }
        while let Some(Located { value: token, pos: _ }) = parser.peek() {
            let arg = Expression::parse(parser)?;
            args.push(arg);
            if let Some(Located { value: Token::ExprOut, pos: _ }) = parser.peek() {
                let Located { value: _, pos: end_pos } = parser.expect()?;
                pos.extend(&end_pos);
                return Ok(Located::new(Self(args), pos));
            }
            if let Some(Located { value: token, pos }) = parser.get() {
                if token != Token::Seperate {
                    return Err(Located::new(format!("expected token {:?} or {:?}, got token {token:?}", Token::Seperate, Token::ExprOut), pos))
                }
            }
        }
        Err(Located::new("unclosed arguments".into(), pos))
    }
}

impl Expression {
    pub fn binary(parser: &mut Parser<Token>, layer: usize) -> ParseResult<Self> {
        let Some(ops) = BinaryOperator::layer(layer) else {
            return Self::unary(parser, 0)
        };
        let mut left = Self::binary(parser, layer + 1)?;
        while let Some(Located { value: token, pos: _ }) = parser.peek() {
            let Some(op) = BinaryOperator::token(token) else {
                break;
            };
            if !ops.contains(&op) {
                break;
            }
            parser.expect()?;
            let right = Self::binary(parser, layer + 1)?;
            let mut pos = left.pos.clone();
            pos.extend(&right.pos);
            left = Located::new(Self::Binary { op, left: Box::new(left), right: Box::new(right) }, pos)
        }
        Ok(left)
    }
    pub fn unary(parser: &mut Parser<Token>, layer: usize) -> ParseResult<Self> {
        let Some(ops) = UnaryOperator::layer(layer) else {
            return Self::call(parser)
        };
        if let Some(Located { value: token, pos: _ }) = parser.peek() {
            if let Some(op) = UnaryOperator::token(token) {
                if ops.contains(&op) {
                    let Located { value: _, mut pos } = parser.expect()?;
                    let right = Self::unary(parser, layer)?;
                    pos.extend(&right.pos);
                    return Ok(Located::new(Self::Unary { op, right: Box::new(right) }, pos))
                }
            }
        }
        Self::unary(parser, layer + 1)
    }
    pub fn call(parser: &mut Parser<Token>) -> ParseResult<Self> {
        let head = Atom::path(parser)?;
        if let Some(Located { value: Token::ExprIn, pos: _ }) = parser.peek() {
            let args = Args::parse(parser)?;
            let mut pos = head.pos.clone();
            pos.extend(&args.pos);
            Ok(Located::new(Self::Call { head, args }, pos))
        } else {
            Ok(head.map(Self::Atom))
        }
    }
    pub fn atom(parser: &mut Parser<Token>) -> ParseResult<Self> {
        Ok(Atom::parse(parser)?.map(Self::Atom))
    }
}
impl Parsable<Token> for Expression {
    fn parse(parser: &mut Parser<Token>) -> ParseResult<Self> {
        Self::binary(parser, 0)
    }
}
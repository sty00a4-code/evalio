use crate::{
    structure::{
        position::Located,
        tokens::*
    },
    tools::lex::*
};

pub fn lex(input: String) -> Result<Vec<Located<Token>>, Located<String>> {
    Lexer::<Token>::new(input).lex()
}

impl Token {
    pub fn ident(ident: String) -> Self {
        match ident.as_str() {
            "true" => Self::Boolean(true),
            "false" => Self::Boolean(false),
            "none" => Self::None,
            _ => Self::Ident(ident)
        }
    }
}
impl Lexable for Token {
    fn step(lexer: &mut Lexer<Self>) -> Result<Option<Located<Self>>, Located<String>> {
        let pos = lexer.pos();
        let Some(c) = lexer.get() else { return Ok(None) };
        match c {
            '(' => { lexer.advance(); Ok(Some(Located::new(Self::ExprIn, pos))) }
            ')' => { lexer.advance(); Ok(Some(Located::new(Self::ExprOut, pos))) }
            '[' => { lexer.advance(); Ok(Some(Located::new(Self::ArrayIn, pos))) }
            ']' => { lexer.advance(); Ok(Some(Located::new(Self::ArrayOut, pos))) }
            '{' => { lexer.advance(); Ok(Some(Located::new(Self::ObjIn, pos))) }
            '}' => { lexer.advance(); Ok(Some(Located::new(Self::ObjOut, pos))) }
            '+' => { lexer.advance(); Ok(Some(Located::new(Self::Add, pos))) }
            '-' => { lexer.advance(); Ok(Some(Located::new(Self::Sub, pos))) }
            '*' => { lexer.advance(); Ok(Some(Located::new(Self::Mul, pos))) }
            '/' => { lexer.advance(); Ok(Some(Located::new(Self::Div, pos))) }
            '^' => { lexer.advance(); Ok(Some(Located::new(Self::Pow, pos))) }
            '%' => { lexer.advance(); Ok(Some(Located::new(Self::Mod, pos))) }
            '.' => { lexer.advance(); Ok(Some(Located::new(Self::Field, pos))) }
            ',' => { lexer.advance(); Ok(Some(Located::new(Self::Seperate, pos))) }
            _ => if let Some(res) = lexer.delimit('"', '"', Some('\\')) {
                let (string, pos) = res?;
                Ok(Some(Located::new(Self::String(string), pos)))
            } else if let Some(res) = lexer.delimit('\'', '\'', Some('\\')) {
                let (string, pos) = res?;
                Ok(Some(Located::new(Self::String(string), pos)))
            } else if let Some((mut number, mut pos)) = lexer.collect_while(&|c| c.is_ascii_digit()) {
                if lexer.get() == Some('.') {
                    number.push('.');
                    pos.extend(&lexer.pos());
                    lexer.advance();
                    if let Some((decimal, other_pos)) = lexer.collect_while(&|c| c.is_ascii_digit()) {
                        number.push_str(&decimal);
                        pos.extend(&other_pos);
                    }
                    match number.parse() {
                        Ok(number) => Ok(Some(Located::new(Self::Float(number), pos))),
                        Err(err) => Err(Located::new(err.to_string(), pos))
                    }
                } else {
                    match number.parse() {
                        Ok(number) => Ok(Some(Located::new(Self::Int(number), pos))),
                        Err(err) => Err(Located::new(err.to_string(), pos))
                    }
                }
            } else if let Some((ident, pos)) = lexer.collect_while(&|c| c.is_alphanumeric() || c == '_') {
                Ok(Some(Located::new(Self::ident(ident), pos)))
            } else {
                Err(Located::new(format!("bad character {:?}", lexer.get().unwrap_or_default()), lexer.pos()))
            }
        }
    }
}
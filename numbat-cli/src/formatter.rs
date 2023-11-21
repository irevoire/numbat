use core::fmt;
use numbat::{tokenize, NumbatError, Token, TokenKind};
use std::fmt::Write;

pub fn run_formatter(input: &str) -> String {
    let mut ret = String::new();

    let tokens = tokenize(input, 0).unwrap();
    let mut token_stream = tokens.into_iter();

    while let Some(token) = token_stream.next() {
        match token.kind {
            // space before and after symbol
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Multiply
            | TokenKind::Power
            | TokenKind::Divide
            | TokenKind::Per
            | TokenKind::Equal
            | TokenKind::Identifier
            | TokenKind::Arrow => {
                if ret.chars().last().map_or(false, |c| c == ' ' || c == '(') {
                    write!(ret, "{} ", token.lexeme).unwrap();
                } else {
                    write!(ret, " {} ", token.lexeme).unwrap();
                }
            }
            // space after
            TokenKind::Comma | TokenKind::Fn | TokenKind::Colon => {
                write!(ret, "{} ", token.lexeme).unwrap();
            }
            TokenKind::LeftParen | TokenKind::RightParen => {
                // We want to remove all the spaces at the end of the line first.
                let to_pop = ret.chars().rev().take_while(|c| *c == ' ').count();
                (0..to_pop).for_each(|_| {
                    ret.pop();
                });
                write!(ret, "{}", token.lexeme).unwrap();
            }
            TokenKind::Newline => {
                // We want to remove all the spaces at the end of the line first.
                let to_pop = ret.chars().rev().take_while(|c| *c == ' ').count();
                (0..to_pop).for_each(|_| {
                    ret.pop();
                });

                // We only need indentation after the equal symbol `=` of the following statements:
                // - variable declaration: let a = [here]
                // - dimension declaration: dimension a = [here]
                // - unit declaration: unit a = [here]
                // - function declaration: fn tamo(param: Input) -> Output = [here]
                // This always happens after an `=` sign thus we can indent only in the
                // case a line ends with the equal sign.
                if ret.chars().last().map_or(false, |c| c == '=') {
                    writeln!(ret).unwrap();
                    write!(ret, "    ").unwrap();
                } else {
                    writeln!(ret).unwrap();
                }
            }
            // Don't modify
            _ => write!(ret, "{}", token.lexeme).unwrap(),
            TokenKind::DoubleColon => todo!(),

            TokenKind::PostfixApply => todo!(),
            TokenKind::UnicodeExponent => todo!(),
            TokenKind::At => todo!(),
            TokenKind::Ellipsis => todo!(),
            TokenKind::ExclamationMark => todo!(),
            TokenKind::EqualEqual => todo!(),
            TokenKind::NotEqual => todo!(),
            TokenKind::LessThan => todo!(),
            TokenKind::GreaterThan => todo!(),
            TokenKind::LessOrEqual => todo!(),
            TokenKind::GreaterOrEqual => todo!(),
            TokenKind::Let => todo!(),
            TokenKind::Dimension => todo!(),
            TokenKind::Unit => todo!(),
            TokenKind::Use => todo!(),
            TokenKind::To => todo!(),
            TokenKind::Bool => todo!(),
            TokenKind::True => todo!(),
            TokenKind::False => todo!(),
            TokenKind::If => todo!(),
            TokenKind::Then => todo!(),
            TokenKind::Else => todo!(),
            TokenKind::String => todo!(),
            TokenKind::Long => todo!(),
            TokenKind::Short => todo!(),
            TokenKind::Both => todo!(),
            TokenKind::None => todo!(),
            TokenKind::ProcedurePrint => todo!(),
            TokenKind::ProcedureAssert => todo!(),
            TokenKind::ProcedureAssertEq => todo!(),
            TokenKind::ProcedureType => todo!(),
            TokenKind::Number => todo!(),
            TokenKind::IntegerWithBase(_) => todo!(),
            TokenKind::StringFixed => todo!(),
            TokenKind::StringInterpolationStart => todo!(),
            TokenKind::StringInterpolationMiddle => todo!(),
            TokenKind::StringInterpolationEnd => todo!(),
            TokenKind::Eof => todo!(),
        }
    }

    ret
}

struct Formatter<TokenStream, Writer>
where
    TokenStream: Iterator<Item = Token>,
    Writer: fmt::Write,
{
    token_stream: TokenStream,
    writer: Writer,
}

impl<TokenStream, Writer> Formatter<TokenStream, Writer>
where
    TokenStream: Iterator<Item = Token>,
    Writer: fmt::Write,
{
    pub fn new(token_stream: TokenStream, writer: Writer) -> Self {
        Self {
            token_stream,
            writer,
        }
    }

    pub fn run(&mut self) {
        while let Some(token)
    }
}

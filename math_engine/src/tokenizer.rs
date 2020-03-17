use crate::token::Token;
use crate::context::{DefaultContext, Context};
use crate::utils::string_tokenizer::{StringTokenizer, TokenizeKind};
use crate::function::Notation;
use crate::error::{Result, Error, ErrorKind};
use crate::num::checked::CheckedNum;
use std::marker::PhantomData;
use std::str::FromStr;

/// Provides a way to retrieve the tokens of an expression.
pub trait Tokenize<N>{
    /// Gets the tokens of the specified expression.
    fn tokenize(&self, expression: &str) -> Result<Vec<Token<N>>>;
}

/// The default `Tokenizer`.
pub struct Tokenizer<'a, N, C = DefaultContext<'a, N>> where C: Context<'a, N> {
    context: &'a C,
    _marker: PhantomData<N>
}

impl <'a, N> Tokenizer<'a, N, DefaultContext<'a, N>> where N: CheckedNum + 'static{
    #[inline]
    pub fn new() -> Self{
        Tokenizer {
            context: DefaultContext::instance(),
            _marker: PhantomData
        }
    }
}

impl <'a, N, C> Tokenizer<'a, N, C> where C: Context<'a, N>, N: FromStr {
    #[inline]
    pub fn with_context(context: &'a C) -> Self{
        Tokenizer {
            context,
            _marker: PhantomData
        }
    }
}

impl <'a, N, C> Tokenize<N> for Tokenizer<'a, N, C> where C: Context<'a, N>, N : FromStr {
    fn tokenize(&self, expression: &str) -> Result<Vec<Token<N>>> {
        const STRING_TOKENIZER : StringTokenizer = StringTokenizer::new(TokenizeKind::RemoveWhiteSpaces);
        const COMMA : &str = ",";
        const WHITESPACE : &str = " ";

        if expression.is_empty(){
            return Err(Error::new(ErrorKind::InvalidExpression, "Expression is empty"));
        }

        let raw_tokens = STRING_TOKENIZER.get_tokens(expression);
        let mut iter = raw_tokens.iter().peekable();
        let mut tokens = Vec::new();
        let mut pos = 0;
        let context = self.context;

        while let Some(string) = iter.next(){
            if is_number(string){
                if context.config().complex_number(){
                    match iter.peek(){
                        Some(s) if *s == "i" => {
                            let mut temp = string.clone();
                            let im = iter.next().unwrap();
                            temp.push_str(im);

                            let n = N::from_str(&temp).ok().expect(&format!("Failed on convert `{}` to a number", temp));
                            tokens.push(Token::Number(n));
                        },
                        _ => {
                            let n = N::from_str(string).ok().expect(&format!("Failed on convert `{}` to a number", string));
                            tokens.push(Token::Number(n));
                        }
                    }
                }
                else{
                    let n = N::from_str(string).ok().expect(&format!("Failed on convert `{}` to a number", string));
                    tokens.push(Token::Number(n));
                }
            }
            else if context.is_variable(string){
                tokens.push(Token::Variable(string.clone()));
            }
            else if context.is_constant(string){
                tokens.push(Token::Constant(string.clone()));
            }
            else if context.is_function(string) {
                tokens.push(Token::Function(string.clone()));
            }
            else if context.is_binary_function(string) || context.is_unary_function(string) {
                let prev = if pos == 0 { None } else { Some(raw_tokens[pos - 1].as_str()) };
                let next = if pos == raw_tokens.len() - 1 { None } else { Some(raw_tokens[pos].as_str()) };

                if is_unary(prev, string, next, context){
                    let operator = string.chars().nth(0).unwrap();
                    tokens.push(Token::UnaryOperator(operator));
                }
                else{
                    //debug_assert!(prev.is_some() && next.is_some(), "Binary operations need 2 operands: {:?} {:?} {:?}", prev, string, next);

                    if prev.is_none() || next.is_none(){
                        return Err(Error::new(
                            ErrorKind::InvalidExpression,
                            format!("Binary operations need 2 operands: {:?} {} {:?}", prev, string, next))
                        );
                    }

                    if string.len() == 1 {
                        let operator = string.chars().nth(0).unwrap();
                        tokens.push(Token::BinaryOperator(operator));
                    }
                    else{
                        tokens.push(Token::InfixFunction(string.clone()));
                    }
                }
            }
            else if string == COMMA{
                tokens.push(Token::Comma);
            }
            else if string == WHITESPACE{
            }
            else if string.len() == 1{
                let c = string.chars().nth(0).unwrap();
                if let Some(symbol) = context.config().get_group_symbol(c){
                    if c == symbol.group_open{
                        tokens.push(Token::GroupingOpen(c));
                    }
                    else{
                        tokens.push(Token::GroupingClose(c));
                    }
                }
                else{
                    tokens.push(Token::Unknown(string.clone()));
                }
            }
            else{
                tokens.push(Token::Unknown(string.clone()));
            }

            pos += 1;
        }

        Ok(tokens)
    }
}

fn is_unary<'a, N>(prev: Option<&str>, cur: &str, next: Option<&str>, context: &impl Context<'a, N>) -> bool{
    if let Some(op) = context.get_unary_function(cur){
        if op.notation() == Notation::Postfix {
            prev.map_or(
                false,
                |s| s == ")" || is_number(s) || context.is_constant(s) || context.is_variable(s)
            )
        }
        else{
            if next.is_none(){ // 10-, (24)+
                return false;
            }

            if prev.is_none(){ // -10, +(25)
                true
            }
            else{
                let prev_str = prev.unwrap();

                // 10+, 2+(2), (4)-10
                if prev_str == ")" || prev_str == "]" || is_number(prev_str) || context.is_variable(prev_str) || context.is_constant(prev_str){
                    return false;
                }

                // 10! - 2
                if context.is_unary_function(&prev_str[..1]) && !context.is_binary_function(&prev_str[..1]){
                    return false;
                }

                // +-, (-, !+
                return if prev_str.len() == 1 {
                    let c = prev_str.chars().last().unwrap();
                    c.is_ascii_punctuation()
                } else {
                    true
                }
            }
        }
    }
    else{
        false
    }
}

fn is_number(value: &str) -> bool {
    if value == "0"{
        return true;
    }

    if value.len() == 0{
        return false;
    }

    let mut has_decimal_point = false;
    let is_signed = value.starts_with("+") || value.starts_with("-");
    let mut iterator = value.chars().enumerate();

    if is_signed && value.len() == 1{
        return false;
    }

    if is_signed {
        iterator.next();
    }

    for item in iterator{
        match item {
            (n, '0') => {
                let starts_with_zero = if is_signed{
                    value[1..].starts_with('0')
                }
                else{
                    value.starts_with('0')
                };

                if !has_decimal_point && starts_with_zero && n > 0 { //+00, 00
                    if let Some(c) = value.chars().nth(n - 1){
                        if c == '0'{
                            return false;
                        }
                    }
                }
            },
            (_, '1'..='9') => {},
            (n, '.') if n < value.len() - 1 => {
                if (is_signed && n > 1) || !is_signed{
                    if has_decimal_point{
                        return false;
                    }
                    else{
                        has_decimal_point = true;
                    }
                }
                else{
                    return false;
                }

            },
            _ => { return false; }
        }
    }

    true
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn is_number_test(){
        assert!(is_number("0"));
        assert!(is_number("6"));
        assert!(is_number("700"));
        assert!(is_number("567"));
        assert!(is_number("1000000.05"));
        assert!(is_number("10.0"));
        assert!(is_number("-102"));
        assert!(is_number("+55"));
        assert!(is_number("+66.4"));
        assert!(is_number("-45.90"));
        assert!(is_number("0.0001"));
        assert!(is_number(".10"));

        assert!(!is_number("00"));
        assert!(!is_number("+00"));
        assert!(!is_number(""));
        assert!(!is_number(" "));
        assert!(!is_number("."));
        assert!(!is_number("+"));
        assert!(!is_number("-"));
        assert!(!is_number("89."));
        assert!(!is_number("10+"));
        assert!(!is_number("20-"));
        assert!(!is_number("+.10"));
        assert!(!is_number("-.25"));
        assert!(!is_number("1..2"));
    }

    #[test]
    fn is_unary_test(){
        let context: &DefaultContext<i64 >= &DefaultContext::new_checked();
        assert!(is_unary(None, "-", Some("5"), context));
        assert!(is_unary(None, "-", Some("Pi"), context));
        assert!(is_unary(Some("("), "-", Some("5"), context));
        assert!(is_unary(Some("("), "-", Some("Pi"), context));
        assert!(is_unary(Some("+"), "-", Some("5"), context));
        assert!(is_unary(Some("+"), "-", Some("E"), context));
        assert!(is_unary(Some("5"), "!", None, context));
        assert!(is_unary(Some("E"), "!", None, context));
        assert!(is_unary(Some(","), "-", Some("5"), context));
        assert!(is_unary(Some("@"), "-", Some("5"), context));

        assert!(!is_unary(Some("3"), "-", Some("5"), context));
        assert!(!is_unary(Some(")"), "-", Some("5"), context));
        assert!(!is_unary(Some("E"), "-", Some("Pi"), context));
        assert!(!is_unary(Some(")"), "-", Some("E"), context));
        assert!(!is_unary(Some(")"), "-", Some("("), context));
    }

    #[test]
    fn tokenize_test(){
        let context: &DefaultContext<i64 >= &DefaultContext::new_checked();
        let tokenizer : Tokenizer<i64> = Tokenizer::with_context(context);
        assert_eq!(&tokenizer.tokenize("2 + 3").unwrap(), &[
            Token::Number(2),
            Token::BinaryOperator('+'),
            Token::Number(3)]);

        assert_eq!(&tokenizer.tokenize("5 * Sin(pi)").unwrap(), &[
            Token::Number(5),
            Token::BinaryOperator('*'),
            Token::Function(String::from("Sin")),
            Token::GroupingOpen('('),
            Token::Constant(String::from("pi")),
            Token::GroupingClose(')')]);

        assert_eq!(&tokenizer.tokenize("10/2 mod 3^2").unwrap(), &[
            Token::Number(10),
            Token::BinaryOperator('/'),
            Token::Number(2),
            Token::InfixFunction(String::from("mod")),
            Token::Number(3),
            Token::BinaryOperator('^'),
            Token::Number(2)]);

        assert_eq!(&tokenizer.tokenize("10! + 2").unwrap(), &[
            Token::Number(10),
            Token::UnaryOperator('!'),
            Token::BinaryOperator('+'),
            Token::Number(2)]);

        assert_eq!(&tokenizer.tokenize("600!").unwrap(), &[
            Token::Number(600),
            Token::UnaryOperator('!')]);
    }
}
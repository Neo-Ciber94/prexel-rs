use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::panic::RefUnwindSafe;
use std::str::FromStr;

use math_engine::error::{Error, ErrorKind};
use math_engine::evaluator::Evaluator;
use math_engine::function::Function;
use math_engine::token::Token::*;
use math_engine::tokenizer::{Tokenize, Tokenizer};
use std::rc::Rc;
use std::error;

pub struct CustomFunction<'a, T>
where
    T: Display + Debug + Clone + FromStr,
{
    function_name: String,
    params: Vec<String>,
    body: String,
    evaluator: Rc<Evaluator<'a, T>>,
    _marker: PhantomData<T>,
}

impl<'a, T> RefUnwindSafe for CustomFunction<'a, T> where T: Display + Debug + Clone + FromStr {}

impl<'a, T> CustomFunction<'a, T>
where
    T: Display + Debug + Clone + FromStr,
{
    #[inline]
    pub fn with_evaluator(
        function_name: String,
        params: Vec<String>,
        body: String,
        evaluator: Rc<Evaluator<'a, T>>,
    ) -> Self {
        CustomFunction {
            function_name,
            params,
            body,
            evaluator,
            _marker: PhantomData,
        }
    }

    pub fn from_str(evaluator: Rc<Evaluator<'a, T>>, s: &str) -> Result<Self, ParseFunctionError> {
        fn check_name(name: &str) -> Result<(), ParseFunctionError> {
            if name.is_empty()
                || name.chars().any(char::is_whitespace)
                || !name.chars().all(char::is_alphanumeric)
            {
                return Err(ParseFunctionError::from(FunctionErrorKind::InvalidName(
                    name.to_string(),
                )));
            }

            Ok(())
        }

        if s.is_empty() {
            return Err(ParseFunctionError::from(FunctionErrorKind::Empty));
        }

        let parts: Vec<&str> = s.split("=").map(|s| s.trim()).collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(ParseFunctionError::from(FunctionErrorKind::InvalidFormat));
        }

        let func = parts[0]; // Contains the function name and params
        let body = parts[1]; // Contains the function body

        if let (Some(paren_open), Some(paren_close)) = (func.find("("), func.find(")")) {
            let function_name = func[..paren_open].to_string();
            let params: Vec<String> = match &func[(paren_open + 1)..paren_close] {
                s if s.is_empty() => Vec::new(),
                s => {
                    let mut temp: Vec<String> = Vec::new();
                    for p in s.split(",").map(|s| s.trim()).map(|s| s.to_string()) {
                        check_name(&p)?;
                        if temp.contains(&p) {
                            return Err(ParseFunctionError::from(
                                FunctionErrorKind::DuplicatedParam(p),
                            ));
                        }

                        temp.push(p);
                    }

                    temp
                }
            };

            Self::check_function_body(&evaluator, &params, body)?;
            Ok(CustomFunction::with_evaluator(
                function_name,
                params,
                body.to_string(),
                evaluator,
            ))
        } else {
            Err(ParseFunctionError::from(FunctionErrorKind::InvalidFormat))
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.function_name
    }

    #[inline]
    pub fn params(&self) -> &[String] {
        &self.params
    }

    #[inline]
    pub fn body(&self) -> &str {
        &self.body
    }

    fn check_function_body(
        evaluator: &Evaluator<T>,
        params: &Vec<String>,
        expr: &str,
    ) -> Result<(), ParseFunctionError> {
        if expr.is_empty() {
            return Err(ParseFunctionError::from(FunctionErrorKind::Empty));
        }

        for p in params {
            if !expr.contains(p) {
                return Err(ParseFunctionError::from(FunctionErrorKind::InvalidParam));
            }
        }

        let context = evaluator.context();
        let tokenizer = Tokenizer::with_context(context);

        match tokenizer.tokenize(expr) {
            Err(_) => Err(ParseFunctionError::from(FunctionErrorKind::InvalidBody)),
            Ok(tokens) => {
                // Gets all the unknown values which should be equals to the number of params.
                let locals = tokens
                    .into_iter()
                    .filter(|t| t.is_unknown())
                    .map(|t| match t {
                        Unknown(s) => s,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<String>>();

                if locals.len() != params.len() {
                    return Err(ParseFunctionError::from(FunctionErrorKind::InvalidBody));
                }

                Ok(())
            }
        }
    }
}

impl<'a, T> Function<T> for CustomFunction<'a, T>
where
    T: Display + Debug + Clone + FromStr,
{
    #[inline]
    fn name(&self) -> &str {
        self.name()
    }

    fn call(&self, args: &[T]) -> math_engine::Result<T> {
        if args.len() != self.params.len() {
            return Err(Error::from(ErrorKind::InvalidArgumentCount));
        }

        let mut expr = self.body.clone();
        for i in 0..self.params.len() {
            expr = expr.replace(&self.params[i], &args[i].to_string())
        }

        self.evaluator.eval(&expr)
    }
}

#[derive(Debug)]
pub struct ParseFunctionError {
    kind: FunctionErrorKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionErrorKind {
    Empty,
    InvalidName(String),
    UnusedParam(String),
    DuplicatedParam(String),
    //InvalidParamName,
    InvalidBody,
    InvalidFormat,
    InvalidParam,
}

impl Display for ParseFunctionError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            FunctionErrorKind::Empty => write!(f, "Empty expression"),
            FunctionErrorKind::InvalidName(ref s) =>
                write!(f, "Expected param names with not whitespaces and alphanumeric: `{}`", s),
            FunctionErrorKind::UnusedParam(ref s) =>
                write!(f, "A param `{}` is not being used in the body of the function", s),
            FunctionErrorKind::DuplicatedParam(ref s) => {
                write!(f, "The param `{}` is duplicated", s)
            }
            FunctionErrorKind::InvalidBody => write!(f, "Invalid function body expression"),
            FunctionErrorKind::InvalidFormat => {
                write!(f, "Invalid format, expected: FunctionName(args, ..) = expr")
            }
            FunctionErrorKind::InvalidParam => write!(f, "Invalid param name"),
        }
    }
}

impl error::Error for ParseFunctionError{}

impl From<FunctionErrorKind> for ParseFunctionError {
    #[inline]
    fn from(kind: FunctionErrorKind) -> Self {
        ParseFunctionError { kind }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn try_from(expr: &str) -> Result<CustomFunction<f64>, ParseFunctionError> {
        let evaluator = Evaluator::new();
        CustomFunction::from_str(Rc::new(evaluator), expr)
    }

    #[test]
    fn from_str_test() {
        let evaluator: Evaluator<f64> = Evaluator::new();
        let func = CustomFunction::from_str(
            Rc::new(evaluator),
            "Add2(x) = x + 2"
        ).unwrap();

        assert_eq!(func.function_name, "Add2");
        assert_eq!(&func.params, &["x".to_string()]);
        assert_eq!(func.body, "x + 2");
    }

    #[test]
    fn from_str_and_eval_test() {
        let evaluator: Evaluator<f64> = Evaluator::new();
        let func = CustomFunction::from_str(
            Rc::new(evaluator),
            "Add2(x) = x + 2"
        ).unwrap();

        assert_eq!(func.call(&[4_f64]), Ok(6_f64));
    }

    #[test]
    fn from_str_error_test() {
        assert!(try_from("").is_err());
        assert!(try_from("Get(x) = y").is_err());
        assert!(try_from("Misplace(x = y").is_err());
        assert!(try_from("Sum(x1, x2)").is_err());
        assert!(try_from("Sum(x1, x2) = ").is_err());
        assert!(try_from("Sum(x, x) = x + x").is_err());
        assert!(try_from("Sum(x1, x2) = x1").is_err());
        assert!(try_from("Sum(x1, x2, x3,) = x1 + x3 + x3").is_err());

        assert!(try_from("GetOne() = 1").is_ok());
        assert!(try_from("Sum(x1, x2) = x2 + x1").is_ok());
        assert!(try_from("Sum(x1, x2, x3) = (x1 + x2) * x3").is_ok());
    }

    #[test]
    fn call_test() {
        let evaluator: Evaluator<f64> = Evaluator::new();
        let func = CustomFunction::with_evaluator(
            "Plus".to_string(),
            vec!["x".to_string(), "y".to_string()],
            "x + y".to_string(),
            Rc::new(evaluator),
        );

        assert_eq!(func.call(&[2_f64, 4_f64]), Ok(6_f64));
    }
}

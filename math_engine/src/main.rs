use std::marker::PhantomData;
use math_engine::evaluator::Evaluate;
use std::str::FromStr;
use std::fmt::{Display, Formatter};

fn main(){

}

pub struct CustomFunction<T>{
    function_name: String,
    params: Vec<String>,
    body: String,
    _marker: PhantomData<T>
}

impl<T> CustomFunction<T>{
    #[inline]
    pub fn new(function_name: String, params: Vec<String>, body: String) -> Self{
        CustomFunction{
            function_name,
            params,
            body,
            _marker: PhantomData
        }
    }

    #[inline]
    pub fn name(&self) -> &str{
        &self.function_name
    }

    #[inline]
    pub fn params(&self) -> &[String]{
        &self.params
    }

    #[inline]
    pub fn body(&self) -> &str{
        &self.body
    }
}

#[derive(Debug)]
pub struct ParseFunctionError {
    kind: FunctionErrorKind
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FunctionErrorKind{
    Empty,
    InvalidName,
    UnusedParam,
    InvalidFormat,
    InvalidParam,
}

impl Display for FunctionErrorKind{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self{
            FunctionErrorKind::Empty =>  write!(f, "Empty expression"),
            FunctionErrorKind::InvalidName => write!(f, "Invalid name, expected not whitespaces and alphanumeric"),
            FunctionErrorKind::UnusedParam => write!(f, "A param is not being used in the body of the function"),
            FunctionErrorKind::InvalidFormat => write!(f, "Invalid format, expected: FunctionName(args, ..) = expr"),
            FunctionErrorKind::InvalidParam => write!(f, "Invalid param name"),
        }
    }
}

impl From<FunctionErrorKind> for ParseFunctionError {
    #[inline]
    fn from(kind: FunctionErrorKind) -> Self {
        ParseFunctionError{
            kind
        }
    }
}

impl<T> FromStr for CustomFunction<T>{
    type Err = ParseFunctionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn check_name(name: &str) -> Result<(), ParseFunctionError>{
            if name.is_empty() ||
                name.chars().any(char::is_whitespace) ||
                !name.chars().all(char::is_alphanumeric){
                return Err(ParseFunctionError::from(FunctionErrorKind::InvalidName));
            }

            Ok(())
        }

        if s.is_empty(){
            return Err(ParseFunctionError::from(FunctionErrorKind::Empty));
        }

        let parts : Vec<&str> = s.split("=")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        if parts.len() != 2{
            return Err(ParseFunctionError::from(FunctionErrorKind::InvalidFormat));
        }

        let func = parts[0];
        let body = parts[1];

        if let (Some(paren_open), Some(paren_close)) = (func.find("("), func.find(")")){
            let function_name = func[..paren_open].to_string();
            let params : Vec<String> = match &func[(paren_open + 1) ..paren_close]{
                s if s.is_empty() => Vec::new(),
                s => {
                    let mut temp = Vec::new();
                    for p in s.split(",").map(|s| s.trim()){
                        check_name(p)?;
                        temp.push(p.to_string());
                    }
                    temp
                }
            };

            for p in &params{
                if !body.contains(p){
                    return Err(ParseFunctionError::from(FunctionErrorKind::InvalidParam));
                }
            }

            Ok(CustomFunction::new(function_name, params, body.to_string()))
        }
        else{
            Err(ParseFunctionError::from(FunctionErrorKind::InvalidFormat))
        }
    }
}
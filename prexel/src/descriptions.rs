use std::fmt::Display;

/// Collection of description for prexel functions and operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Description {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Plus,
    Abs,
    Sum,
    Prod,
    Avg,
    Factorial,
    Pow,
    Max,
    Min,
    Floor,
    Ceil,
    Truncate,
    Round,
    Sign,
    Sqrt,
    Cbrt,
    Exp,
    Ln,
    Log,
    Rand,
    ToRadians,
    ToDegrees,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Sinh,
    Cosh,
    Tanh,
    Csch,
    Sech,
    Coth,
    ASin,
    ACos,
    ATan,
    ACsc,
    ASec,
    ACot,
    ASinh,
    ACosh,
    ATanh,
    ACsch,
    ASech,
    ACoth,
}

impl Description {
    /// Returns a string representation of the description.
    pub fn as_str(&self) -> &'static str {
        use Description::*;

        match self {
            Add => "Add two values",
            Sub => "Subtract two values",
            Mul => "Multiply two values",
            Div => "Divide two values",
            Mod => "Modulo two values",
            Neg => "Negate a value",
            Plus => "Applies +n to a value",
            Abs => "Absolute value of a values",
            Sum => "Gets the sum of all the values",
            Prod => "Gets the product of all the values",
            Avg => "Gets the average of all the values",
            Factorial => "Gets the factorial of a value",
            Pow => "Gets the power of a value",
            Max => "Gets the maximum of all the values",
            Min => "Gets the minimum of all the values",
            Floor => "Rounds a value down",
            Ceil => "Rounds a value up",
            Truncate => "Gets the integer part of a number",
            Round => "Gets the round of a value",
            Sign => "Gets the sign of a value as -1, 0, or 1",
            Sqrt => "Gets the square root of a value",
            Cbrt => "Gets the cubic root of a value",
            Exp => "Gets the exponential of a value",
            Ln => "Gets the natural logarithm of a value",
            Log => "Gets the logarithm of a value",
            Rand => "Gets a random value: between 0 and 1, 0..MAX or a range",
            ToRadians => "Gets the radian value of a degree value",
            ToDegrees => "Gets the degree value of a radian value",
            Sin => "Gets the sine of a value",
            Cos => "Gets the cosine of a value",
            Tan => "Gets the tangent of a value",
            Csc => "Gets the cosecant of a value",
            Sec => "Gets the secant of a value",
            Cot => "Gets the cotangent of a value",
            Sinh => "Gets the hyperbolic sine of a value",
            Cosh => "Gets the hyperbolic cosine of a value",
            Tanh => "Gets the hyperbolic tangent of a value",
            Csch => "Gets the hyperbolic cosecant of a value",
            Sech => "Gets the hyperbolic secant of a value",
            Coth => "Gets the hyperbolic cotangent of a value",
            ASin => "Gets the arc sine of a value",
            ACos => "Gets the arc cosine of a value",
            ATan => "Gets the arc tangent of a value",
            ACsc => "Gets the arc cosecant of a value",
            ASec => "Gets the arc secant of a value",
            ACot => "Gets the arc cotangent of a value",
            ASinh => "Gets the hyperbolic arc sine of a value",
            ACosh => "Gets the hyperbolic arc cosine of a value",
            ATanh => "Gets the hyperbolic arc tangent of a value",
            ACsch => "Gets the hyperbolic arc cosecant of a value",
            ASech => "Gets the hyperbolic arc secant of a value",
            ACoth => "Gets the hyperbolic arc cotangent of a value",
        }
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Description> for &'static str {
    fn from(description: Description) -> Self {
        description.as_str()
    }
}

impl From<Description> for String {
    fn from(description: Description) -> Self {
        description.as_str().to_string()
    }
}
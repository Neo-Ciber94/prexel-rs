
/// Trait for approximate a value to the closest.
pub trait Approx{
    /// Approximates the value.
    fn approx(&self) -> Self;
    /// Approximates the value by the specified delta.
    fn approx_by(&self, delta: &Self) -> Self;
}

/// Trait for check if 2 values are equals using the specified delta.
pub trait ApproxEq {
    /// Checks if the two values are equals using the given delta.
    fn approx_eq(&self, other: &Self, delta: &Self) -> bool;
}
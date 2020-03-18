use num_traits::{ToPrimitive, FromPrimitive};
use crate::function::Function;
use crate::error::{Error, ErrorKind};
use crate::error::Result;
use rand::random;
use crate::ops::math::try_to_float;

pub struct RandFunction;
impl<N: ToPrimitive + FromPrimitive> Function<N> for RandFunction{
    #[inline]
    fn name(&self) -> &str {
        "random"
    }

    fn call(&self, args: &[N]) -> Result<N> {
        match args.len(){
            0 => N::from_f64(random::<f64>()).ok_or(Error::from(ErrorKind::Overflow)),
            1 => {
                let max = try_to_float(&args[0])?;
                N::from_f64(random::<f64>() * max)
                    .ok_or(Error::from(ErrorKind::Overflow))
            },
            2 => {
                let min = try_to_float(&args[0])?;
                let max = try_to_float(&args[1])?;
                let value = min + ((max - min) *  random::<f64>());
                N::from_f64(value)
                    .ok_or(Error::from(ErrorKind::Overflow))
            },
            _ => Err(Error::from(ErrorKind::InvalidArgumentCount))
        }
    }
}
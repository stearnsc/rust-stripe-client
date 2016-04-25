use serde;

#[derive(Debug, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R)
}

use Either::*;
impl<L, R> serde::Deserialize for Either<L, R>
    where L: serde::Deserialize, R: serde::Deserialize
{
    fn deserialize<D: serde::Deserializer>(deserializer: &mut D) -> Result<Either<L, R>, D::Error> {
        L::deserialize(deserializer).map(Left)
            .or_else(|_| R::deserialize(deserializer).map(Right))
    }
}

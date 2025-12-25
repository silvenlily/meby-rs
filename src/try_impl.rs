use core::convert::Infallible;
use core::ops::{ControlFlow, FromResidual, Try};
use crate::Meby;

impl<Res, Err> Try for Meby<Res, Err> {
    type Output = Res;
    type Residual = Meby<Infallible, Err>;

    fn from_output(output: Self::Output) -> Self {
        Meby::Yes(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Meby::Yes(res) => ControlFlow::Continue(res),
            Meby::Oops(err) => ControlFlow::Break(Meby::Oops(err)),
            Meby::Nope => ControlFlow::Break(Meby::Nope),
        }
    }
}

impl<Res, Err1: Into<Err2>, Err2> FromResidual<Meby<Infallible, Err1>> for Meby<Res, Err2> {
    fn from_residual(residual: Meby<Infallible, Err1>) -> Self {
        match residual {
            Meby::Yes(_) => unreachable!(),
            Meby::Oops(err) => Meby::Oops(err.into()),
            Meby::Nope => Meby::Nope,
        }
    }
}

impl<Res, Err> FromResidual<Option<Infallible>> for Meby<Res, Err> {
    fn from_residual(_: Option<Infallible>) -> Self {
        Meby::Nope
    }
}

impl<Res, Err1: Into<Err2>, Err2> FromResidual<Result<Infallible, Err1>> for Meby<Res, Err2> {
    fn from_residual(res: Result<Infallible, Err1>) -> Self {
        match res {
            Err(e) => Meby::Oops(e.into()),
            Ok(_) => unreachable!(),
        }
    }
}
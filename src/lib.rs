#![cfg_attr(feature = "try_trait_v2", feature(try_trait_v2))]

#[cfg(feature = "try_trait_v2")]
pub mod try_impl;

use core::mem;

/**
 * Can be: `Yes<Res>` `Oops<Err>` or `Nope`
 * Effectively an easier to handle `Option<Result>` or `Result<Option>`
 * can be converted back and forth between `Option<Result>` and `Result<Option>` using `from()` or `into()`
 */
pub enum Meby<Res, Err> {
    Yes(Res),
    Oops(Err),
    Nope,
}

impl<Res, Err> Meby<Res, Err> {
    /// you don't need a doc comment for this
    pub fn is_yes(&self) -> bool {
        match self {
            Meby::Yes(_) => true,
            _ => false,
        }
    }

    /// you don't need a doc comment for this
    pub fn is_oops(&self) -> bool {
        match self {
            Meby::Oops(_) => true,
            _ => false,
        }
    }

    /// you don't need a doc comment for this
    pub fn is_nope(&self) -> bool {
        match self {
            Meby::Nope => true,
            _ => false,
        }
    }

    /// returns the contents of self if self is yes, otherwise panics
    pub fn unwrap(self) -> Res {
        match self {
            Meby::Yes(res) => res,
            Meby::Oops(_) => panic!("unwrap called on meby value that turned out to be an error."),
            Meby::Nope => panic!("unwrap called on meby value that turned out to be nope."),
        }
    }

    /// returns the contents of self if self is oops, otherwise panics
    pub fn unwrap_oops(self) -> Err {
        match self {
            Meby::Yes(_) => panic!("unwrap_err called on meby value that turned out to be a yes."),
            Meby::Oops(err) => err,
            Meby::Nope => panic!("unwrap_err called on meby value that turned out to be nope."),
        }
    }

    /// replaces the contents of self with the contents of default if self is not yes
    pub fn yes_or(&mut self, default: Res) -> &mut Self {
        match self {
            Meby::Oops(_) => {
                _ = mem::replace(self, Meby::Yes(default));
            }
            Meby::Nope => {
                _ = mem::replace(self, Meby::Yes(default));
            }
            _ => {}
        }
        self
    }

    /// returns the contents of self if self is a yes, otherwise returns default
    pub fn unwrap_or(self, default: Res) -> Res {
        match self {
            Meby::Yes(res) => res,
            Meby::Oops(_) => default,
            Meby::Nope => default,
        }
    }

    /// replaces the contents of self with the contents of other, returning the old contents of self
    pub fn swap(&mut self, other: Self) -> Self {
        mem::replace(self, other)
    }

    /// effectively calls `into()` on the inner value  
    /// you don't have to change both values as a type is always `from` to itself
    pub fn into_inner<NewRes: From<Res>, NewErr: From<Err>>(self) -> Meby<NewRes, NewErr> {
        match self {
            Meby::Yes(res) => Meby::Yes(res.into()),
            Meby::Oops(err) => Meby::Oops(err.into()),
            Meby::Nope => Meby::Nope,
        }
    }
}

impl<Res, Err> From<Option<Res>> for Meby<Res, Err> {
    fn from(value: Option<Res>) -> Self {
        match value {
            None => Meby::Nope,
            Some(val) => Meby::Yes(val),
        }
    }
}

impl<Res, Err> From<Result<Res, Err>> for Meby<Res, Err> {
    fn from(value: Result<Res, Err>) -> Self {
        match value {
            Ok(val) => Meby::Yes(val),
            Err(err) => Meby::Oops(err),
        }
    }
}

impl<Res, Err> From<Option<Result<Res, Err>>> for Meby<Res, Err> {
    fn from(value: Option<Result<Res, Err>>) -> Self {
        match value {
            None => Meby::Nope,
            Some(Ok(val)) => Meby::Yes(val),
            Some(Err(val)) => Meby::Oops(val),
        }
    }
}

impl<Res, Err> From<Result<Option<Res>, Err>> for Meby<Res, Err> {
    fn from(value: Result<Option<Res>, Err>) -> Self {
        match value {
            Ok(Some(val)) => Meby::Yes(val),
            Ok(None) => Meby::Nope,
            Err(err) => Meby::Oops(err),
        }
    }
}

impl<Res, Err> From<Meby<Res, Err>> for Result<Option<Res>, Err> {
    fn from(value: Meby<Res, Err>) -> Self {
        match value {
            Meby::Yes(val) => Ok(Some(val)),
            Meby::Oops(err) => Err(err),
            Meby::Nope => Ok(None),
        }
    }
}

impl<Res, Err> From<Meby<Res, Err>> for Option<Result<Res, Err>> {
    fn from(value: Meby<Res, Err>) -> Self {
        match value {
            Meby::Yes(val) => Some(Ok(val)),
            Meby::Oops(err) => Some(Err(err)),
            Meby::Nope => None,
        }
    }
}

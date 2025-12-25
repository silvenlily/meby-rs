# Meby

[[repo]](https://github.com/silvenlily/meby-rs)

Provides a `Meby<Res,Err>` type that can be: `Yes<Res>` `Oops<Err>` or `Nope`  
Effectively an easier to handle `Option<Result>` or `Result<Option>`  
`Meby` can be converted back and forth between `Option<Result>` and `Result<Option>` 

Meby supports the `?` operator, but requires [the unstable try_trait_v2 feature](https://github.com/rust-lang/rust/issues/84277)
this is gated behind the try_trait_v2 feature which is enabled by default. Disable default features to prevent this.

The variants of `Meby` (`Yes`, `Nope` & `Oops`) are named as such to prevent conflicting with `Ok` `Err` `Some` and `None`.
Yes the names are a little silly. I like it that way.

Meby has no dependencies and is no_std compatible (only uses things from core).

Meby currently has all the convince functions I use regularly.
It does not have all the convenience functions that Option or Result have.
Feel free to make a pull request.
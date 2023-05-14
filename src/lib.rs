#![no_std]

#[cfg(feature = "windows")]
pub mod windows;

use core::fmt::Debug;

pub trait ResultImpl<T, E> {
    fn unwrap_unreachable(self) -> T
    where
        E: Debug;

    fn expect_unreachable(self, msg: &str) -> T
    where
        E: Debug;
}

impl<T, E> ResultImpl<T, E> for Result<T, E> {
    fn unwrap_unreachable(self) -> T
    where
        E: Debug,
    {
        self.unwrap_or_else(|e| {
            unreachable!(
                "called `Result::unwrap_unreachable` on an `Err` value: {:?}",
                e
            )
        })
    }

    fn expect_unreachable(self, msg: &str) -> T
    where
        E: Debug,
    {
        self.unwrap_or_else(|e| unreachable!("{msg}: {e:?}"))
    }
}

pub trait OptionImpl<T> {
    fn unwrap_unreachable(self) -> T;

    fn expect_unreachable(self, msg: &str) -> T;
}

impl<T> OptionImpl<T> for Option<T> {
    fn unwrap_unreachable(self) -> T {
        self.unwrap_or_else(|| {
            unreachable!("called `Option::unwrap_unreachable` on a `None` value")
        })
    }

    fn expect_unreachable(self, msg: &str) -> T {
        self.unwrap_or_else(|| unreachable!("{msg}"))
    }
}

#[cfg(test)]
mod __tests {
    use super::*;

    #[test]
    #[should_panic = "internal error: entered unreachable code: called \
                      `Result::unwrap_unreachable` on an `Err` value: \"lol\""]
    fn test_result_unwrap() {
        Err::<(), &str>("lol").unwrap_unreachable();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: oops: \"lol\""]
    fn test_result_expect() {
        Err::<(), &str>("lol").expect_unreachable("oops");
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: called \
                      `Option::unwrap_unreachable` on a `None` value"]
    fn test_option_unwrap() {
        None::<()>.unwrap_unreachable();
    }

    #[test]
    #[should_panic = "internal error: entered unreachable code: oops"]
    fn test_option_expect() {
        None::<()>.expect_unreachable("oops");
    }
}

use windows::Win32::Foundation::BOOL;
use windows::Win32::Foundation::BOOLEAN;
use windows::Win32::Foundation::VARIANT_BOOL;

macro_rules! __gen_impl {
    ($($i:ident),+$(,)?) => {
        ::paste::paste! {
            use $crate::ResultImpl as _;

            $(
                #[allow(nonstandard_style)]
                pub trait [<$i Impl>] {
                    fn unwrap_unreachable(self);

                    fn expect_unreachable(self, msg: &::core::primitive::str);
                }

                impl [<$i Impl>] for $i {
                    fn unwrap_unreachable(self) {
                        self.ok().unwrap_unreachable()
                    }

                    fn expect_unreachable(self, msg: &::core::primitive::str) {
                        self.ok().expect_unreachable(msg)
                    }
                }

            )+

            #[cfg(test)]
            #[allow(nonstandard_style)]
            mod __tests {
                use super::*;

                $(
                    #[test]
                    #[should_panic = "internal error: entered unreachable code: called \
                                      `Result::unwrap_unreachable` on an `Err` value: Error { \
                                      code: HRESULT(0x00000000), message: \"Success.\" }"]
                    fn [<test_ $i _unwrap>]() {
                        $i(0).unwrap_unreachable();
                    }

                    #[test]
                    #[should_panic = "internal error: entered unreachable code: oops: Error { \
                                      code: HRESULT(0x00000000), message: \"Success.\" }"]
                    fn [<test_ $i _expect>]() {
                        $i(0).expect_unreachable("oops");
                    }
                )+
            }
        }
    };
}

__gen_impl! {
    BOOL,
    BOOLEAN,
    VARIANT_BOOL,
}

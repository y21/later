#![cfg_attr(not(feature = "async"), no_std)]

#[cfg(feature = "async")]
#[doc(hidden)]
pub use smol as __priv_smol;

#[cfg(feature = "async")]
#[doc(hidden)]
#[macro_export]
macro_rules! defer_async {
    (|$($var:ident : $t:ty),*| async $code:expr) => {
        $crate::defer!(|$($var: $t),*| {
            $crate::__priv_smol::block_on(async {
                $code
            });
        });
    }
}

#[cfg(not(feature = "async"))]
#[doc(hidden)]
#[macro_export]
macro_rules! defer_async {
    (|$($var:ident : $t:ty),*| async $code:expr) => {
        ::core::compile_error!("Cannot have async drop guards without the async feature flag!");
    };
}

#[macro_export]
macro_rules! defer {
    (|| $code:expr) => { $crate::defer!(| | $code); };
    (|$($var:ident : $t:ty),*| async $code:expr) => {
        $crate::defer_async!(|$($var: $t),*| async $code);
    };
    (|$($var:ident : $t:ty),*| $code:expr) => {
        struct GeneratedDropGuardDoNotUseThis {
            // Wrapping the type in Option allows access to owned $t in Drop code
            // This is required because Drop::drop takes &mut self, so we cannot move out of it
            $(#[allow(dead_code)] $var: ::core::option::Option<$t>),*
        }
        impl ::core::ops::Drop for GeneratedDropGuardDoNotUseThis {
            fn drop(&mut self) {
                $(let $var = self.$var.take().unwrap();)*
                $code
            }
        }
        let _drop_guard = GeneratedDropGuardDoNotUseThis {
            $($var: ::core::option::Option::Some(::core::clone::Clone::clone(&$var))),*
        };
        let _drop_guard = ::core::pin::Pin::new(&_drop_guard);
    };
    (|$($var:ident $(: $t:ty)? ),*| $(async)? $code:expr) => {
        ::std::compile_error!("Type inference for drop guard state is currently unimplemented. Please explicitly annotate types.");
    };
    ($($token:tt)*) => {
        $crate::defer!(|| { $($token)* });
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {
        let x = 32;
        defer!(|x: i32| {
            println!("World - {}", x);
        });
        println!("Hello");
    }
}

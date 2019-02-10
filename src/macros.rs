#[doc(hidden)]
#[macro_export]
macro_rules! op_default {
    ($func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl $bound for $cls {
            type Output = Self;

            fn $func(mut self, _rhs: Self) -> Self {
                for i in 0..self.size() {
                    self[i] $op _rhs[i];
                }
                self
            }
        }
    };
    ($type:ty, $func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl<I: Into<$type>> $bound<I> for $cls {
            type Output = Self;

            fn $func(mut self, _rhs: I) -> Self {
                self $op _rhs;
                self
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! op_assign {
    ($func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl $bound for $cls {
            fn $func(&mut self, _rhs: Self) {
                for i in 0..self.size() {
                    self[i] $op _rhs[i];
                }
            }
        }
    };
    ($type:ty, $func:ident, $bound:ident, $op:tt, $cls:ident) => {
        impl<I: Into<$type>> $bound<I> for $cls {
            fn $func(&mut self, _rhs: I) {
                let k = _rhs.into();
                for i in 0..self.size() {
                    self[i] $op k;
                }
            }
        }
    };
}
pub trait OverflowingFrom<T> where Self: Sized{
    fn overflowing_from(val: T) -> (Self, bool);
}

pub trait OverflowingTo: Sized {
    fn overflowing_to<T: OverflowingFrom<Self>>(self) -> (T, bool) {
        T::overflowing_from(self)
    }
}

macro_rules! impl_overflowing {
    ($($self:ident from $($from:ty)*),+) => {
        $(
            $(impl OverflowingFrom<$from> for $self {
                fn overflowing_from(val: $from) -> (Self, bool) {
                    let overflow = val < Self::MIN as _ || val > Self::MAX as _;
                    (val as _, overflow)
                }
            })*
            impl OverflowingTo for $self {}
        )+
    };
}

impl_overflowing! {
    i8  from u8  i16 u16 i32 u32 i64 u64,
    i16 from u16 i32 u32 i64 u64,
    i32 from u32 i64 u64,
    i64 from,

    u8  from i8  i16 u16 i32 u32 i64 u64,
    u16 from i16 i32 u32 i64 u64,
    u32 from i32 i64 u64,
    u64 from
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflowing_u32_to_u8() {
        assert_eq!(u8::overflowing_from(345u32), (89u8, true));
        assert_eq!(345u32.overflowing_to::<u8>(), (89u8, true));

        assert_eq!(u8::overflowing_from(123u32), (123u8, false));
        assert_eq!(123u32.overflowing_to::<u8>(), (123u8, false));
    }
}

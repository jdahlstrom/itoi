pub trait OverflowFrom<T> where Self: Sized{
    fn overflow_from(val: T) -> (Self, bool);
}

pub trait OverflowTo: Sized {
    fn overflow_to<T: OverflowFrom<Self>>(self) -> (T, bool) {
        T::overflow_from(self)
    }
}

macro_rules! impl_overflow {
    ($($self:ident from $($from:ty)*),+) => {
        $(
            $(impl OverflowFrom<$from> for $self {
                fn overflow_from(val: $from) -> (Self, bool) {
                    let overflow = val < Self::MIN as _ || val > Self::MAX as _;
                    (val as _, overflow)
                }
            })*
            impl OverflowTo for $self {}
        )+
    };
}

impl_overflow! {
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
    fn overflow_u32_to_u8() {
        assert_eq!(u8::overflow_from(345u32), (89u8, true));
        assert_eq!(345u32.overflow_to::<u8>(), (89u8, true));

        assert_eq!(u8::overflow_from(123u32), (123u8, false));
        assert_eq!(123u32.overflow_to::<u8>(), (123u8, false));
    }
}

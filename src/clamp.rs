pub trait ClampFrom<T> {
    fn clamp_from(val: T) -> Self;
}

pub trait ClampTo: Sized {
    fn clamp_to<T: ClampFrom<Self>>(self) -> T {
        T::clamp_from(self)
    }
}

macro_rules! impl_clamp {
    ($($self:ident from $($from:ty)*),+) => {
        $(
            $(impl ClampFrom<$from> for $self {
                fn clamp_from(val: $from) -> Self {
                    let res = Self::try_from(val);

                    if let Ok(res) = res {
                        res
                    }
                }
            })*
            impl ClampTo for $self {}
        )+
    };
}

impl_clamp! {
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
    fn clamp_u32_to_u8() {
        assert_eq!(u8::clamp_from(345u32), 255u8);
        assert_eq!(345u32.clamp_to::<u8>(), 255u8);
    }
    #[test]
    fn clamp_i8_to_u8() {
        assert_eq!(u8::clamp_from(127i8), 127u8);
        assert_eq!((-127i8).clamp_to::<u8>(), 0u8);
    }
    #[test]
    fn clamp_u8_to_i8() {
        assert_eq!(i8::clamp_from(127u8), 127i8);
        assert_eq!((128u8).clamp_to::<i8>(), 127i8);
    }
}

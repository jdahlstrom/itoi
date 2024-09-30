pub trait WrapFrom<T> {
    fn wrap_from(val: T) -> Self;
}

pub trait WrapTo: Sized {
    fn wrap_to<T: WrapFrom<Self>>(self) -> T {
        T::wrap_from(self)
    }
}

macro_rules! impl_wrap {
    ($($self:ident from $($from:ty)*),+) => {
        $(
            $(impl WrapFrom<$from> for $self {
                fn wrap_from(val: $from) -> Self {
                    val as _ // TODO
                }
            })*
            impl WrapTo for $self {}
        )+
    };
}

impl_wrap! {
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
    fn wrap_u32_to_u8() {
        assert_eq!(u8::wrap_from(345u32), 89u8);
        assert_eq!(345u32.wrap_to::<u8>(), 89u8);
    }
}

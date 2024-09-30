pub trait WidenFrom<T> {
    fn widen_from(val: T) -> Self;
}

pub trait WidenTo: Sized {
    fn widen_to<T: WidenFrom<Self>>(self) -> T {
        T::widen_from(self)
    }
}

macro_rules! impl_widen {
    ($($self:ident from $($from:ty)*),+) => {
        $(
            $(impl WidenFrom<$from> for $self {
                fn widen_from(val: $from) -> Self {
                    val.into()
                }
            })*
            impl WidenTo for $self {}
        )+
    };
}

impl_widen! {
    i64 from i8 u8 i16 u16 i32 u32,
    i32 from i8 u8 i16 u16,
    i16 from i8 u8,
    i8 from,

    u64 from u8 u16 u32,
    u32 from u8 u16,
    u16 from u8,
    u8 from,

    f64 from f32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widen() {
        assert_eq!(u32::widen_from(12u8), 12u32);
        assert_eq!(12u8.widen_to::<u32>(), 12u32);
    }
}

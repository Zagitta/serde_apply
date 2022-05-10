use crate::SerdeApply;

macro_rules! impl_apply_basic {
    ( $x:ty ) => {
        impl<'de> SerdeApply for $x {
            type PartialType = Option<$x>;
            #[inline]
            fn apply(&mut self, with: Self::PartialType) {
                with.map(|val| *self = val);
            }
        }
    };
    ( $( $x:ty ),* ) => {
        $(
            impl_apply_basic!($x);
        )*
    }
}

impl_apply_basic!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, String);

impl<'de, T> SerdeApply for Option<T>
where
    T: SerdeApply + Default,
{
    type PartialType = Option<T::PartialType>;
    #[inline]
    fn apply(&mut self, with: Self::PartialType) {
        with.map(|val| match self {
            Some(this) => this.apply(val),
            None => {
                let mut this = T::default();
                this.apply(val);
                *self = Some(this);
            }
        });
    }
}

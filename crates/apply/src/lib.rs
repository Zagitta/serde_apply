mod impls;
use serde::{Deserialize, Deserializer};

pub use serde_apply_macros::SerdeApply;
pub trait SerdeApply {
    type PartialType;
    fn apply(&mut self, with: Self::PartialType);
}

pub fn apply<'de, T, DE>(data: &mut T, de: DE) -> Result<(), DE::Error>
where
    T: Deserialize<'de> + SerdeApply,
    T::PartialType: Deserialize<'de>,
    DE: Deserializer<'de>,
{
    data.apply(<T::PartialType as Deserialize<'de>>::deserialize(de)?);
    Ok(())
}

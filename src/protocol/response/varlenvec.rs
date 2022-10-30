use serde::{
    de::{DeserializeSeed, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt::Formatter, marker::PhantomData};

pub struct VarLenVec<T> {
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> VarLenVec<T> {
    pub fn new(len: usize) -> Self {
        Self { len, _marker: PhantomData }
    }
}

impl<'de, T> Visitor<'de> for VarLenVec<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "x")
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut res = Vec::with_capacity(self.len);

        for _ in 0..self.len {
            let x = seq
                .next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

            res.push(x);
        }

        return Ok(res);
    }
}

impl<'de, T> DeserializeSeed<'de> for VarLenVec<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(self.len, self)
    }
}

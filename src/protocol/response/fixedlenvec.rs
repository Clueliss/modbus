use serde::{
    de::{DeserializeSeed, SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use std::{fmt::Formatter, marker::PhantomData};

/// DeserializSeed for a Vec with fixed length
pub struct FixedLenVec<T> {
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> FixedLenVec<T> {
    pub fn new(len: usize) -> Self {
        Self { len, _marker: PhantomData }
    }
}

impl<'de, T> Visitor<'de> for FixedLenVec<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{} of length {}", std::any::type_name::<Vec<T>>(), self.len)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        debug_assert!(seq.size_hint().is_some_and(|h| h == self.len));

        let mut res = Vec::with_capacity(self.len);
        while let Some(x) = seq.next_element()? {
            res.push(x);
        }

        Ok(res)
    }
}

impl<'de, T> DeserializeSeed<'de> for FixedLenVec<T>
where
    T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(self.len, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{response::FixedLenVec, BINCODE_OPTS};
    use bincode::Options;

    #[test]
    fn sanity_check() {
        let mut serialized = Vec::new();

        BINCODE_OPTS.serialize_into(&mut serialized, &4usize).unwrap();
        BINCODE_OPTS.serialize_into(&mut serialized, &1u8).unwrap();
        BINCODE_OPTS.serialize_into(&mut serialized, &2u8).unwrap();
        BINCODE_OPTS.serialize_into(&mut serialized, &3u8).unwrap();
        BINCODE_OPTS.serialize_into(&mut serialized, &4u8).unwrap();
        BINCODE_OPTS.serialize_into(&mut serialized, &0u8).unwrap();

        let mut serialized = &serialized[..];

        let data_len: usize = BINCODE_OPTS.deserialize_from(&mut serialized).unwrap();
        let deserialized: Vec<u8> = BINCODE_OPTS
            .deserialize_from_seed(FixedLenVec::<u8>::new(data_len), &mut serialized)
            .unwrap();
        assert_eq!(deserialized, vec![1, 2, 3, 4]);
    }
}

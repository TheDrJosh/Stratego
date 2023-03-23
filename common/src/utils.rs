use std::marker::PhantomData;

use serde::{
    de::{self, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SendibleArray<T: ~const Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize>(
    pub [T; SIZE],
);

impl<T: ~const Default + Serialize + for<'de> Deserialize<'de>, const SIZE: usize> Serialize
    for SendibleArray<T, SIZE>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(SIZE))?;
        for piece in &self.0 {
            seq.serialize_element(&piece)?;
        }
        seq.end()
    }
}

struct BoardVisitor<T: Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize>(
    PhantomData<*const T>,
);

impl<'de, T: ~const Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize> Visitor<'de>
    for BoardVisitor<T, SIZE>
{
    type Value = SendibleArray<T, SIZE>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a SendibleArray")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut arr = [const { None }; SIZE];

        let mut i = 0;
        while let Some(value) = seq.next_element()? {
            arr[i] = Some(value);
            i += 1;
        }

        if i != SIZE {
            return Err(de::Error::invalid_length(
                i,
                &(format!("Length of {}", SIZE).as_str()),
            ));
        }

        Ok(SendibleArray(arr.map(|x| x.unwrap())))
    }
}

impl<'de, T: ~const Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize>
    Deserialize<'de> for SendibleArray<T, SIZE>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(BoardVisitor::<T, SIZE>(PhantomData::<*const T>))
    }
}

impl<T: ~const Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize> Default
    for SendibleArray<T, SIZE>
{
    fn default() -> Self {
        let t = [false; SIZE];
        Self(t.map(|_| T::default()))
    }
}

impl<T: ~const Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize>
    std::ops::Index<usize> for SendibleArray<T, SIZE>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: ~const Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize>
    std::ops::IndexMut<usize> for SendibleArray<T, SIZE>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: ~const Default + Serialize + for<'a> Deserialize<'a>, const SIZE: usize> From<[T; SIZE]>
    for SendibleArray<T, SIZE>
{
    fn from(value: [T; SIZE]) -> Self {
        SendibleArray(value)
    }
}

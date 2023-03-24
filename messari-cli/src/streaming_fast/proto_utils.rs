use std::fmt::Debug;

pub(crate) trait FromSignedVarint: Sized
{
    fn from_signed_varint(data: &mut &[u8]) -> Option<Self>;
}

impl<T: Default + TryFrom<i64>> FromSignedVarint for T
    where
        T::Error: Debug,
{
    fn from_signed_varint(data: &mut &[u8]) -> Option<Self>
    {
        u64::from_unsigned_varint(data).map(|u| {
            let signed: i64 = unsafe { std::mem::transmute(u) };
            signed.try_into().unwrap()
        })
    }
}

pub(crate) trait FromUnsignedVarint: Sized
{
    fn from_unsigned_varint(data: &mut &[u8]) -> Option<Self>;
}

impl<T: Default + TryFrom<u64>> FromUnsignedVarint for T
    where
        T::Error: Debug,
{
    fn from_unsigned_varint(data: &mut &[u8]) -> Option<Self>
    {
        let mut result = 0u64;
        let mut idx = 0;
        loop {
            if idx >= data.len() {
                return None;
            }

            let b = data[idx];
            let value = (b & 0x7f) as u64;
            result += value << (idx * 7);

            idx += 1;
            if b & 0x80 == 0 {
                break;
            }
        }

        let result = T::try_from(result).expect("Out of range");
        *data = &data[idx..];
        Some(result)
    }
}
use std::fmt::Debug;

pub(crate) fn get_file_size_string(file_size: usize) -> String {
    if file_size < 1024 { // (<100B)
        format!("{}B", file_size)
    } else if file_size < 100*1024 { // (<100KB)
        format!("{:.2}KB", (file_size as f64)/1024_f64)
    } else if file_size < 1024*1024 { // (<1MB)
        format!("{}KB", file_size)
    } else if file_size < 100*1024*1024 { // (<100MB)
        format!("{:.2}MB", (file_size as f64)/(1024_f64*1024_f64))
    } else if file_size < 1024*1024*1024 { // (<1GB)
        format!("{}MB", file_size)
    } else if file_size < 100*1024*1024*1024 { // (<100GB)
        format!("{:.2}GB", (file_size as f64)/(1024_f64*1024_f64*1024_f64))
    } else { // (>100GB)
        // We are expecting to produce file around the block size of
        // 128MB so some of the above is already overkill here..
        format!("{:+e}B", file_size as f64)
    }
}

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
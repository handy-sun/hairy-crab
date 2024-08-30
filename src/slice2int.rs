use std::convert::TryInto;

pub trait ReadInteger<T> {
	fn from_le_bytes(data: &[u8]) -> T;
	fn from_be_bytes(data: &[u8]) -> T;
}

macro_rules! impl_read_integer {
    ($($t:ty),+) => {
        $(impl ReadInteger<$t> for $t {
            fn from_le_bytes(data: &[u8]) -> $t {
                <$t>::from_le_bytes(data.try_into().unwrap())
            }
            fn from_be_bytes(data: &[u8]) -> $t {
                <$t>::from_be_bytes(data.try_into().unwrap())
            }
        })+
    }
}

impl_read_integer!(u8, i16, i32, u32, i64);

fn read_le_int<T: ReadInteger<T>>(data: &[u8]) -> T {
	T::from_le_bytes(&data[..std::mem::size_of::<T>()])
}

fn main() -> Result<(), &'static str> {
	let slice = b"\xfe\xff\x00\x00";
	// let slice = &[254, 255, 0, 0];

	let var1 = read_le_int::<i16>(slice);
	let var2 = read_le_int::<u32>(slice);
	println!("{}, {}", var1, var2);
	Ok(())
}

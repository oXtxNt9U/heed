use std::borrow::Cow;
use std::{mem, ptr};

use zerocopy::{LayoutVerified, AsBytes, FromBytes, Unaligned};
use crate::{BytesEncode, BytesDecode};
use crate::types::aligned_to;

pub struct UnalignedSlice<T>(std::marker::PhantomData<T>);

impl<T> BytesEncode for UnalignedSlice<T> where T: AsBytes + Unaligned {
    type EItem = [T];

    fn bytes_encode(item: &Self::EItem) -> Option<Cow<[u8]>> {
        Some(Cow::Borrowed(<[T] as AsBytes>::as_bytes(item)))
    }
}

impl<'a, T: 'a> BytesDecode<'a> for UnalignedSlice<T> where T: FromBytes + Unaligned + Copy {
    type DItem = &'a [T];

    fn bytes_decode(bytes: &'a [u8]) -> Option<Self::DItem> {
        LayoutVerified::<_, [T]>::new_slice_unaligned(bytes).map(LayoutVerified::into_slice)
    }
}
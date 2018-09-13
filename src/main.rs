#[macro_use]
extern crate scroll;

use scroll::ctx::SizeWith;
use scroll::{LE, Pread, Pwrite};

#[derive(Debug, PartialEq, Eq, Pread, Pwrite, SizeWith)]
struct A {
    pub y: u64,
    pub x: u32,
}

#[derive(Debug, PartialEq, Eq, Pread, SizeWith)]
struct B {
    pub a: A,
}

impl <'a> ::scroll::ctx::TryIntoCtx<::scroll::Endian> for &'a B {
    type
    Error
    =
    ::scroll::Error;
    type
    Size
    =
    usize;
    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], ctx: ::scroll::Endian)
     -> ::scroll::export::result::Result<Self::Size, Self::Error> {
        use ::scroll::Pwrite;
        let offset = &mut 0;
        dst.gwrite_with(&self.a, offset, ctx)?;
        Ok(*offset)
    }
}
impl ::scroll::ctx::TryIntoCtx<::scroll::Endian> for B {
    type
    Error
    =
    ::scroll::Error;
    type
    Size
    =
    usize;
    #[inline]
    fn try_into_ctx(self, dst: &mut [u8], ctx: ::scroll::Endian)
     -> ::scroll::export::result::Result<Self::Size, Self::Error> {
        (&self).try_into_ctx(dst, ctx)
    }
}

fn main() {
    let b = B { a: A { y: 1, x: 2 } };
    let size = B::size_with(&LE);
    let mut bytes = vec![0; size];
    let written = bytes.pwrite_with(&b, 0, LE).unwrap();
    let mut read = 0;
    let b2: B = bytes.gread_with(&mut read, LE).unwrap();
    assert_eq!(written, size);
    assert_eq!(read, size);
    assert_eq!(b, b2);
}

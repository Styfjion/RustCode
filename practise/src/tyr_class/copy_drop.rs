use std::fmt::{Debug, Formatter};
use std::slice;

#[derive(Copy, Clone)]
struct RawBuffer {
    ptr: *mut u8,
    len: usize
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();
        Self {
            len: slice.len(),
            ptr: Box::into_raw(slice) as *mut u8
        }
    }
}

impl Debug for RawBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p} {data:?}", self.ptr)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {buf:?}");
}

fn main() {
    let data = vec![1, 2, 3, 4];
    let buf: RawBuffer = data.into();
    use_buffer(buf);
    println!("buf: {buf:?}");
}
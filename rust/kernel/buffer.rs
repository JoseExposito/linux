use core::fmt;

pub struct Buffer<'a> {
    slice: &'a mut [u8],
    pos: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        Buffer {
            slice,
            pos: 0,
        }
    }

    pub fn bytes_written(&self) -> usize {
        self.pos
    }
}

impl<'a> fmt::Write for Buffer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() > self.slice.len() - self.pos {
            Err(fmt::Error)
        } else {
            self.slice[self.pos..self.pos + s.len()].copy_from_slice(s.as_bytes());
            self.pos += s.len();
            Ok(())
        }
    }
}

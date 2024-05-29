use std::{mem, ops::{Deref, DerefMut}, ptr};
use super::raw::RawVec;

pub struct Vec<T>{
    buf: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            buf: RawVec::new(),
            len: 0,
        }
    }
    pub fn cap(&self) -> usize {
        self.buf.cap
    }
    pub fn push(&mut self, v: T) {
        if self.len == self.buf.cap {
            self.buf.grow();
        }
        unsafe {
            ptr::write(self.buf.ptr.as_ptr(), v);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.buf.cap == 0 {
            None
        } else {
            unsafe {
                let result = ptr::read(self.buf.ptr.as_ptr());
                self.len -= 1;
                Some(result)
            }
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.buf.cap {
            self.buf.grow();
        }
        unsafe {
            ptr::copy(
                self.buf.ptr.as_ptr().add(index),
                self.buf.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.buf.ptr.as_ptr().add(index), value);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        assert!(index <= self.len, "index out of bounds");
        if self.len == 0 {
            return None;
        }
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.buf.ptr.as_ptr().add(index));
            ptr::copy(
                self.buf.ptr.as_ptr().add(index+1),
                self.buf.ptr.as_ptr().add(self.len),
                self.len - index,
            );
            Some(result)
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop(){}
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.buf.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.buf.ptr.as_ptr(), self.len)
        }
    }
}

pub struct IntoIter<T> {
    _buf: RawVec<T>,
    start: *const T,
    end: *const T,
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T>{
        let buf = unsafe {ptr::read(&self.buf)};
        let len = self.len;
        IntoIter {
            start: buf.ptr.as_ptr(),
            end: if buf.cap == 0 {
                buf.ptr.as_ptr()
            } else {
                unsafe {
                    buf.ptr.as_ptr().add(len)
                }
            },
            _buf: buf,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        unsafe {
            let result = ptr::read(self.start);
            self.start = self.start.offset(1);
            Some(result)
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            return None;
        }
        unsafe {
            let result = ptr::read(self.end);
            self.end = self.end.offset(-1);
            Some(result)
        }
    }
}
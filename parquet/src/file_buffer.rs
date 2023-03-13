use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::io::Write;
use std::mem;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct FileBuffer {
    file_data: Arc<Mutex<Vec<u8>>>
}

impl FileBuffer {
    pub(crate) fn new() -> FileBuffer {
        FileBuffer {
            file_data: Arc::new(Mutex::new(vec![])),
        }
    }

    pub(crate) fn get_data(mut self) -> Vec<u8> {
        mem::take(self.file_data.lock().unwrap().deref_mut())
    }
}

impl Write for FileBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file_data.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}


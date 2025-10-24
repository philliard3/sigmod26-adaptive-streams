pub use spaghetto as devec;
pub mod async_operators;
pub mod async_query_builder;
pub mod basic_pooling;
pub mod caching;
pub mod chroma_utils;
pub mod expression;
pub mod global_logger;
pub mod operators;
pub mod preclassifier_lang;
pub mod query_builder;
pub mod scheduler;
pub mod ws_types;

pub use operators::*;
pub use ws_types::*;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

const CHANNEL_TIMEOUT_MILLIS: u64 = 3;
const MAX_TIMEOUTS: usize = 500;

pub type TupleVec = smallvec::SmallVec<Tuple>;

pub struct FrameReader {
    backing: Vec<Bytes>,
    current: usize,
    index_in_current: usize,
}

impl FrameReader {
    pub fn new(backing: Vec<Bytes>) -> Self {
        FrameReader {
            backing,
            current: 0,
            index_in_current: 0,
        }
    }
    pub fn push(&mut self, bytes: Bytes) {
        self.backing.push(bytes);
    }
}

use std::io::Read;
impl Read for FrameReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut amount_to_read = buf.len();
        let mut bytes_read = 0;
        while amount_to_read > 0 {
            if self.current == self.backing.len() {
                break;
            }
            let current_buffer = &self.backing[self.current];
            let bytes_taken = Read::read(&mut &current_buffer[self.index_in_current..], buf)?;
            bytes_read += bytes_taken;
            amount_to_read -= bytes_taken;
            self.index_in_current += bytes_taken;
            if self.index_in_current >= current_buffer.len() {
                self.current += 1;
                self.index_in_current = 0;
                continue;
            }
            if bytes_taken == 0 {
                // our source is empty
                break;
            }
        }
        Ok(bytes_read)
    }
}

// wrapper type to allow any std::fmt::Write to be used as a std::io::Write
pub struct WriteWrapper<T>(pub T);

impl<T: std::fmt::Write> std::io::Write for WriteWrapper<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let string_form = std::str::from_utf8(buf);
        let string_err_mapped =
            string_form.map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        self.0
            .write_str(string_err_mapped)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// wrapper type to use serde_json to serialize a type, rather than its Debug impl
pub struct SerdeJson<T>(pub T);

impl<T> std::fmt::Debug for SerdeJson<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_writer(WriteWrapper(f), &self.0).map_err(|_e| std::fmt::Error)?;
        Ok(())
    }
}

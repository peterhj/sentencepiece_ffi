extern crate libc;

use crate::ffi::{
    SentencePieceProcessor as CSentencePieceProcessor,
    sentencepiece_processor_new,
    sentencepiece_processor_free,
    sentencepiece_processor_from_serialized_proto,
    sentencepiece_processor_bos_id,
    sentencepiece_processor_eos_id,
    sentencepiece_processor_pad_id,
    sentencepiece_processor_unk_id,
    sentencepiece_processor_is_unknown,
    sentencepiece_processor_num_pieces,
    sentencepiece_processor_id_to_piece,
    sentencepiece_processor_piece_to_id,
    sentencepiece_processor_decode,
    sentencepiece_processor_decode16,
    sentencepiece_processor_encode,
    sentencepiece_processor_encode16,
};

use libc::{c_char, c_int};

use std::ffi::{c_void, CString};
use std::fs::{File};
use std::io::{Read};
use std::path::{Path};

pub mod ffi;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SentencePieceError {
    File,
    IO,
    New,
    Proto(c_int),
    Utf8,
    Decode(c_int),
    Encode,
}

#[repr(C)]
pub struct CBuf<T> {
    pub ptr: *const T,
    pub len: usize,
}

impl<T> Drop for CBuf<T> {
    fn drop(&mut self) {
        assert!(!self.ptr.is_null());
        unsafe { libc::free(self.ptr as *mut c_void); }
    }
}

impl<T> CBuf<T> {
    pub fn as_ref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

/// Sentence piece tokenizer.
///
/// Instances of `SentencePieceProcessor` can be used to tokenizer a
/// sentence using a sentencepiece model.
#[derive(Debug)]
pub struct SentencePieceProcessor {
    inner: *mut CSentencePieceProcessor,
}

impl Drop for SentencePieceProcessor {
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { sentencepiece_processor_free(self.inner) }
    }
}

impl SentencePieceProcessor {
    pub fn from_dir<P: AsRef<Path>>(path: P) -> Result<Self, SentencePieceError> {
        let mut path = path.as_ref().to_owned();
        path.push("tokenizer.model");
        let mut f = File::open(&path).map_err(|_| SentencePieceError::File)?;
        let mut data = Vec::new();
        f.read_to_end(&mut data).map_err(|_| SentencePieceError::IO)?;
        SentencePieceProcessor::from_model(&data)
    }

    pub fn from_model(data: &[u8]) -> Result<Self, SentencePieceError> {
        let inner = unsafe { sentencepiece_processor_new() };
        if inner.is_null() {
            return Err(SentencePieceError::New);
        }
        let spp = SentencePieceProcessor{inner};
        let status = unsafe {
            sentencepiece_processor_from_serialized_proto(
                spp.inner,
                data.as_ptr() as *const c_char,
                data.len(),
            )
        };
        if status != 0 {
            /*let c_error = match FromPrimitive::from_i32(result) {
                Some(error) => error,
                None => unreachable!(),
            };*/
            return Err(SentencePieceError::Proto(status));
        }
        Ok(spp)
    }

    pub fn unk_id(&self) -> c_int {
        let unk_id = unsafe { sentencepiece_processor_unk_id(self.inner) };
        // unk_id must always be present.
        assert!(unk_id >= 0);
        unk_id
    }

    pub fn bos_id(&self) -> Option<c_int> {
        let bos_id = unsafe { sentencepiece_processor_bos_id(self.inner) };
        if bos_id < 0 {
            None
        } else {
            Some(bos_id)
        }
    }

    pub fn eos_id(&self) -> Option<c_int> {
        let eos_id = unsafe { sentencepiece_processor_eos_id(self.inner) };
        if eos_id < 0 {
            None
        } else {
            Some(eos_id)
        }
    }

    pub fn pad_id(&self) -> Option<c_int> {
        let pad_id = unsafe { sentencepiece_processor_pad_id(self.inner) };
        if pad_id < 0 {
            None
        } else {
            Some(pad_id)
        }
    }

    pub fn is_unknown_id(&self, piece_id: c_int) -> bool {
        unsafe { sentencepiece_processor_is_unknown(self.inner, piece_id) != 0 }
    }

    pub fn num_pieces(&self) -> Option<c_int> {
        let n = unsafe { sentencepiece_processor_num_pieces(self.inner) };
        if n < 0 {
            None
        } else {
            Some(n)
        }
    }

    pub fn id_to_piece(&self, piece_id: c_int) -> Result<String, SentencePieceError> {
        let mut piece_ptr: *const i8 = std::ptr::null();
        let mut piece_len: usize = 0;
        unsafe { sentencepiece_processor_id_to_piece(
            self.inner,
            piece_id,
            &mut piece_ptr,
            &mut piece_len,
        ) };
        assert!(!piece_ptr.is_null());
        // NB: the piece is a borrowed const char str.
        let piece_buf = unsafe { std::slice::from_raw_parts(piece_ptr as *const u8, piece_len) };
        String::from_utf8(piece_buf.to_owned()).map_err(|_| SentencePieceError::Utf8)
    }

    /// Get the identifier of a sentence piece.
    pub fn piece_to_id(&self, piece: &str) -> c_int {
        let c_piece = CString::new(piece.as_bytes()).unwrap();
        unsafe { sentencepiece_processor_piece_to_id(self.inner, c_piece.as_ptr()) }
    }

    /// Decode a sentence from piece identifiers.
    pub fn decode(&self, pieces: &[c_int]) -> Result<CBuf<u8>, SentencePieceError> {
        let mut decoded: *mut c_char = std::ptr::null_mut();
        let mut decoded_len: usize = 0;
        let status = unsafe {
            sentencepiece_processor_decode(
                self.inner,
                pieces.as_ptr(),
                pieces.len(),
                &mut decoded,
                &mut decoded_len,
            )
        };
        if decoded.is_null() {
            return Err(SentencePieceError::Decode(status));
        }
        let c_buf = CBuf{ptr: decoded as *const u8, len: decoded_len};
        if status != 0 {
            /*let c_error = match FromPrimitive::from_i32(status) {
                Some(error) => error,
                None => unreachable!(),
            };*/
            return Err(SentencePieceError::Decode(status));
        }
        /*let decoded_string = String::from_utf8(c_buf.as_ref().to_owned())
            .map_err(|_| SentencePieceError::Utf8)?;
        Ok(decoded_string)*/
        Ok(c_buf)
    }

    pub fn decode16(&self, pieces: &[u16]) -> Result<CBuf<u8>, SentencePieceError> {
        let mut decoded: *mut c_char = std::ptr::null_mut();
        let mut decoded_len: usize = 0;
        let status = unsafe {
            sentencepiece_processor_decode16(
                self.inner,
                pieces.as_ptr(),
                pieces.len(),
                &mut decoded,
                &mut decoded_len,
            )
        };
        if decoded.is_null() {
            return Err(SentencePieceError::Decode(status));
        }
        let c_buf = CBuf{ptr: decoded as *const u8, len: decoded_len};
        if status != 0 {
            return Err(SentencePieceError::Decode(status));
        }
        Ok(c_buf)
    }

    /// Encode a sentence as sentence pieces and their identifiers.
    pub fn encode(&self, sentence: &str) -> Result<CBuf<c_int>, SentencePieceError> {
        let mut encoded: *mut c_int = std::ptr::null_mut();
        let mut encoded_len: usize = 0;
        unsafe {
            sentencepiece_processor_encode(
                self.inner,
                sentence.as_ptr() as *const c_char,
                sentence.as_bytes().len(),
                &mut encoded,
                &mut encoded_len,
            );
        }
        if encoded.is_null() {
            return Err(SentencePieceError::Encode);
        }
        Ok(CBuf{ptr: encoded, len: encoded_len})
    }

    pub fn encode16(&self, sentence: &str) -> Result<CBuf<u16>, SentencePieceError> {
        let mut encoded: *mut u16 = std::ptr::null_mut();
        let mut encoded_len: usize = 0;
        unsafe {
            sentencepiece_processor_encode16(
                self.inner,
                sentence.as_ptr() as *const c_char,
                sentence.as_bytes().len(),
                &mut encoded,
                &mut encoded_len,
            );
        }
        if encoded.is_null() {
            return Err(SentencePieceError::Encode);
        }
        Ok(CBuf{ptr: encoded, len: encoded_len})
    }
}

// sentencepiece is thread-safe:
// https://github.com/google/sentencepiece/issues/207

unsafe impl Send for SentencePieceProcessor {}
unsafe impl Sync for SentencePieceProcessor {}

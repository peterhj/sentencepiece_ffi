extern crate libc;
extern crate smol_str;

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
    sentencepiece_processor_encode16_with_suffix,
    sentencepiece_processor_encode16_with_prefix,
    sentencepiece_processor_encode16_with_prefix_suffix,
};

use libc::{c_char, c_int};
use smol_str::{SmolStr};

use std::ffi::{c_void, CString};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::fs::{File};
use std::io::{Read};
use std::ops::{Deref};
use std::path::{Path};
use std::str::{from_utf8};

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
pub struct SentencePieceBuffer<T> {
    ptr: *const T,
    len: usize,
}

impl<T> Drop for SentencePieceBuffer<T> {
    fn drop(&mut self) {
        assert!(!self.ptr.is_null());
        unsafe { libc::free(self.ptr as *mut c_void); }
    }
}

impl<T> Deref for SentencePieceBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T: Debug> Debug for SentencePieceBuffer<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Debug::fmt(self.as_ref(), f)
    }
}

impl<T> SentencePieceBuffer<T> {
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

    pub fn unk_tok16(&self) -> u16 {
        let t = self.unk_id();
        assert!(t >= 0 && t <= u16::max_value() as _);
        t as _
    }

    pub fn bos_id(&self) -> Option<c_int> {
        let bos_id = unsafe { sentencepiece_processor_bos_id(self.inner) };
        if bos_id < 0 {
            None
        } else {
            Some(bos_id)
        }
    }

    pub fn bos_tok16(&self) -> Option<u16> {
        let t = self.bos_id()?;
        assert!(t >= 0 && t <= u16::max_value() as _);
        Some(t as _)
    }

    pub fn eos_id(&self) -> Option<c_int> {
        let eos_id = unsafe { sentencepiece_processor_eos_id(self.inner) };
        if eos_id < 0 {
            None
        } else {
            Some(eos_id)
        }
    }

    pub fn eos_tok16(&self) -> Option<u16> {
        let t = self.eos_id()?;
        assert!(t >= 0 && t <= u16::max_value() as _);
        Some(t as _)
    }

    pub fn pad_id(&self) -> Option<c_int> {
        let pad_id = unsafe { sentencepiece_processor_pad_id(self.inner) };
        if pad_id < 0 {
            None
        } else {
            Some(pad_id)
        }
    }

    pub fn pad_tok16(&self) -> Option<u16> {
        let t = self.pad_id()?;
        assert!(t >= 0 && t <= u16::max_value() as _);
        Some(t as _)
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

    pub fn vocab_len(&self) -> Option<usize> {
        Some(self.num_pieces()? as _)
    }

    pub fn id_to_piece<'this>(&'this self, piece_id: c_int) -> Result<SmolStr, SentencePieceError> {
        let mut piece_ptr: *const c_char = std::ptr::null();
        let mut piece_len: usize = 0;
        unsafe { sentencepiece_processor_id_to_piece(
            self.inner,
            piece_id,
            &mut piece_ptr,
            &mut piece_len,
        ) };
        assert!(!piece_ptr.is_null());
        // NB: the piece is a borrowed const char str.
        let piece_buf: &'this [u8] = unsafe { std::slice::from_raw_parts(piece_ptr as *const u8, piece_len) };
        let piece_str = from_utf8(piece_buf).map_err(|_| SentencePieceError::Utf8)?;
        Ok(piece_str.into())
    }

    pub fn get16(&self, tok: u16) -> Result<SmolStr, SentencePieceError> {
        self.id_to_piece(tok as _)
    }

    /// Get the identifier of a sentence piece.
    pub fn piece_to_id(&self, piece: &str) -> c_int {
        let c_piece = CString::new(piece.as_bytes()).unwrap();
        unsafe { sentencepiece_processor_piece_to_id(self.inner, c_piece.as_ptr()) }
    }

    /// Decode a sentence from piece identifiers.
    pub fn decode(&self, pieces: &[c_int]) -> Result<SentencePieceBuffer<u8>, SentencePieceError> {
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
        let c_buf = SentencePieceBuffer{ptr: decoded as *const u8, len: decoded_len};
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

    pub fn decode16(&self, pieces: &[u16]) -> Result<SentencePieceBuffer<u8>, SentencePieceError> {
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
        let c_buf = SentencePieceBuffer{ptr: decoded as *const u8, len: decoded_len};
        if status != 0 {
            return Err(SentencePieceError::Decode(status));
        }
        Ok(c_buf)
    }

    /// Encode a sentence as sentence pieces and their identifiers.
    pub fn encode(&self, sentence: &str) -> Result<SentencePieceBuffer<c_int>, SentencePieceError> {
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
        Ok(SentencePieceBuffer{ptr: encoded, len: encoded_len})
    }

    pub fn encode16(&self, sentence: &str) -> Result<SentencePieceBuffer<u16>, SentencePieceError> {
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
        Ok(SentencePieceBuffer{ptr: encoded, len: encoded_len})
    }

    pub fn encode16_with_suffix(&self, sentence: &str, suffix_tok: u16) -> Result<SentencePieceBuffer<u16>, SentencePieceError> {
        let mut encoded: *mut u16 = std::ptr::null_mut();
        let mut encoded_len: usize = 0;
        unsafe {
            sentencepiece_processor_encode16_with_suffix(
                self.inner,
                sentence.as_ptr() as *const c_char,
                sentence.as_bytes().len(),
                suffix_tok,
                &mut encoded,
                &mut encoded_len,
            );
        }
        if encoded.is_null() {
            return Err(SentencePieceError::Encode);
        }
        Ok(SentencePieceBuffer{ptr: encoded, len: encoded_len})
    }

    pub fn encode16_with_prefix(&self, sentence: &str, prefix_tok: u16) -> Result<SentencePieceBuffer<u16>, SentencePieceError> {
        let mut encoded: *mut u16 = std::ptr::null_mut();
        let mut encoded_len: usize = 0;
        unsafe {
            sentencepiece_processor_encode16_with_prefix(
                self.inner,
                sentence.as_ptr() as *const c_char,
                sentence.as_bytes().len(),
                prefix_tok,
                &mut encoded,
                &mut encoded_len,
            );
        }
        if encoded.is_null() {
            return Err(SentencePieceError::Encode);
        }
        Ok(SentencePieceBuffer{ptr: encoded, len: encoded_len})
    }

    pub fn encode16_with_prefix_suffix(&self, sentence: &str, prefix_tok: u16, suffix_tok: u16) -> Result<SentencePieceBuffer<u16>, SentencePieceError> {
        let mut encoded: *mut u16 = std::ptr::null_mut();
        let mut encoded_len: usize = 0;
        unsafe {
            sentencepiece_processor_encode16_with_prefix_suffix(
                self.inner,
                sentence.as_ptr() as *const c_char,
                sentence.as_bytes().len(),
                prefix_tok,
                suffix_tok,
                &mut encoded,
                &mut encoded_len,
            );
        }
        if encoded.is_null() {
            return Err(SentencePieceError::Encode);
        }
        Ok(SentencePieceBuffer{ptr: encoded, len: encoded_len})
    }
}

// sentencepiece is thread-safe:
// https://github.com/google/sentencepiece/issues/207

unsafe impl Send for SentencePieceProcessor {}
unsafe impl Sync for SentencePieceProcessor {}

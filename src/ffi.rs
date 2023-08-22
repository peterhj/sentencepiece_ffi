#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
pub struct SentencePieceProcessor {
    _unused: [u8; 0],
}
extern "C" {
    pub fn sentencepiece_processor_decode(
        spp: *mut SentencePieceProcessor,
        pieces: *const ::std::os::raw::c_int,
        pieces_len: usize,
        decoded: *mut *mut ::std::os::raw::c_char,
        decoded_len: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sentencepiece_processor_decode16(
        spp: *mut SentencePieceProcessor,
        pieces: *const u16,
        pieces_len: usize,
        decoded: *mut *mut ::std::os::raw::c_char,
        decoded_len: *mut usize,
    ) -> ::std::os::raw::c_int;
}
/*extern "C" {
    pub fn sentencepiece_processor_decode_pieces(
        spp: *mut SentencePieceProcessor,
        pieces: *const *const ::std::os::raw::c_char,
        pieces_len: usize,
        decoded: *mut *mut ::std::os::raw::c_uchar,
        decoded_len: *mut usize,
    ) -> ::std::os::raw::c_int;
}*/
extern "C" {
    pub fn sentencepiece_processor_encode(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        sentence_len: usize,
        encoded: *mut *mut ::std::os::raw::c_int,
        encoded_len: *mut usize,
    );
}
extern "C" {
    pub fn sentencepiece_processor_encode16(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        sentence_len: usize,
        encoded: *mut *mut u16,
        encoded_len: *mut usize,
    );
}
extern "C" {
    pub fn sentencepiece_processor_encode16_with_suffix(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        sentence_len: usize,
        suffix_tok: u16,
        encoded: *mut *mut u16,
        encoded_len: *mut usize,
    );
}
extern "C" {
    pub fn sentencepiece_processor_encode16_with_prefix(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        sentence_len: usize,
        prefix_tok: u16,
        encoded: *mut *mut u16,
        encoded_len: *mut usize,
    );
}
extern "C" {
    pub fn sentencepiece_processor_encode16_with_prefix_suffix(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        sentence_len: usize,
        prefix_tok: u16,
        suffix_tok: u16,
        encoded: *mut *mut u16,
        encoded_len: *mut usize,
    );
}
/*extern "C" {
    pub fn sentencepiece_processor_encode_as_serialized_proto(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        sentence_len: usize,
        len: *mut usize,
    ) -> *mut ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn sentencepiece_processor_sample_encode_as_serialized_proto(
        spp: *mut SentencePieceProcessor,
        sentence: *const ::std::os::raw::c_char,
        sentence_len: usize,
        len: *mut usize,
        nbest: usize,
        alpha: f32,
    ) -> *mut ::std::os::raw::c_uchar;
}*/
extern "C" {
    pub fn sentencepiece_processor_new() -> *mut SentencePieceProcessor;
}
extern "C" {
    pub fn sentencepiece_processor_from_serialized_proto(
        spp: *mut SentencePieceProcessor,
        data: *const ::std::os::raw::c_char,
        len: usize,
    ) -> ::std::os::raw::c_int;
}
/*extern "C" {
    pub fn sentencepiece_processor_to_serialized_proto(
        spp: *mut SentencePieceProcessor,
        len: *mut usize,
    ) -> *mut ::std::os::raw::c_uchar;
}*/
/*extern "C" {
    pub fn sentencepiece_processor_load(
        spp: *mut SentencePieceProcessor,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}*/
extern "C" {
    pub fn sentencepiece_processor_free(spp: *mut SentencePieceProcessor);
}
extern "C" {
    pub fn sentencepiece_processor_bos_id(spp: *mut SentencePieceProcessor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sentencepiece_processor_eos_id(spp: *mut SentencePieceProcessor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sentencepiece_processor_is_unknown(spp: *mut SentencePieceProcessor, id: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sentencepiece_processor_pad_id(spp: *mut SentencePieceProcessor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sentencepiece_processor_id_to_piece(
        spp: *mut SentencePieceProcessor,
        piece_id: ::std::os::raw::c_int,
        piece: *mut *const ::std::os::raw::c_char,
        piece_len: *mut usize,
    );
}
extern "C" {
    pub fn sentencepiece_processor_piece_to_id(
        spp: *mut SentencePieceProcessor,
        piece: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sentencepiece_processor_num_pieces(spp: *mut SentencePieceProcessor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sentencepiece_processor_unk_id(spp: *mut SentencePieceProcessor) -> ::std::os::raw::c_int;
}

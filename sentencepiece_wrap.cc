//#include <cstdint>
//#include <cstdlib>
//#include <cstring>
#include <type_traits>
#include <vector>
#include <assert.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include <sentencepiece_processor.h>

using absl::string_view;
using sentencepiece::SentencePieceProcessor;
using sentencepiece::SentencePieceText;

// Inspired by:
// https://stackoverflow.com/a/14589519
template<typename E>
constexpr auto to_underlying_type(E e) -> typename std::underlying_type<E>::type 
{
   return static_cast<typename std::underlying_type<E>::type>(e);
}

extern "C" {

SentencePieceProcessor *sentencepiece_processor_new() {
  return new SentencePieceProcessor();
}

void sentencepiece_processor_free(SentencePieceProcessor *sentencepiece_processor) {
  delete sentencepiece_processor;
}

int sentencepiece_processor_from_serialized_proto(SentencePieceProcessor *sentencepiece_processor, char const *data, size_t len) {
  auto status = sentencepiece_processor->LoadFromSerializedProto(string_view(data, len));
  return to_underlying_type(status.code());
}

int sentencepiece_processor_unk_id(SentencePieceProcessor *sentencepiece_processor) {
  return sentencepiece_processor->unk_id();
}

int sentencepiece_processor_bos_id(SentencePieceProcessor *sentencepiece_processor) {
  return sentencepiece_processor->bos_id();
}

int sentencepiece_processor_eos_id(SentencePieceProcessor *sentencepiece_processor) {
  return sentencepiece_processor->eos_id();
}

int sentencepiece_processor_pad_id(SentencePieceProcessor *sentencepiece_processor) {
  return sentencepiece_processor->pad_id();
}

int sentencepiece_processor_is_unknown(SentencePieceProcessor *sentencepiece_processor, int piece_id) {
  return sentencepiece_processor->IsUnknown(piece_id);
}

int sentencepiece_processor_num_pieces(SentencePieceProcessor *sentencepiece_processor) {
  return sentencepiece_processor->GetPieceSize();
}

void sentencepiece_processor_id_to_piece(SentencePieceProcessor *sentencepiece_processor, int piece_id, char const **piece, size_t *piece_len) {
  const std::string& s = sentencepiece_processor->IdToPiece(piece_id);
  *piece = s.data();
  *piece_len = s.size();
}

int sentencepiece_processor_piece_to_id(SentencePieceProcessor *sentencepiece_processor, char const *piece) {
  return sentencepiece_processor->PieceToId(piece);
}

int sentencepiece_processor_decode(SentencePieceProcessor *sentencepiece_processor, int const *pieces, size_t pieces_len, char **decoded, size_t *decoded_len) {
    std::vector<int> int_pieces;
    int_pieces.reserve(pieces_len);

    for (int const *p = pieces; p != pieces + pieces_len; ++p) {
        int_pieces.push_back(static_cast<int>(*p));
    }

    std::string decoded_string;
    auto status = sentencepiece_processor->Decode(int_pieces, &decoded_string);

    size_t len = decoded_string.size();
    *decoded_len = len;
    *decoded = (char *)malloc(len);
    memcpy(*decoded, decoded_string.data(), len);

    return to_underlying_type(status.code());
}

int sentencepiece_processor_decode16(SentencePieceProcessor *sentencepiece_processor, uint16_t const *pieces, size_t pieces_len, char **decoded, size_t *decoded_len) {
    std::vector<int> int_pieces;
    int_pieces.reserve(pieces_len);

    for (uint16_t const *p = pieces; p != pieces + pieces_len; ++p) {
        int_pieces.push_back(static_cast<int>(*p));
    }

    std::string decoded_string;
    auto status = sentencepiece_processor->Decode(int_pieces, &decoded_string);

    size_t len = decoded_string.size();
    *decoded_len = len;
    *decoded = (char *)malloc(len);
    memcpy(*decoded, decoded_string.data(), len);

    return to_underlying_type(status.code());
}

void sentencepiece_processor_encode(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, int **encoded, size_t *encoded_len) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  std::vector<int> ids = sentencepiece_processor->EncodeAsIds(sentence_view);

  size_t len = ids.size();
  *encoded_len = len;
  *encoded = (int *)malloc(sizeof(int) * len);
  memcpy(*encoded, ids.data(), sizeof(int) * len);

  return;
}

void sentencepiece_processor_encode16(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, uint16_t **encoded, size_t *encoded_len) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  std::vector<int> ids = sentencepiece_processor->EncodeAsIds(sentence_view);

  size_t len = ids.size();
  *encoded_len = len;

  uint16_t *buf = (uint16_t *)malloc(sizeof(uint16_t) * len);
  *encoded = buf;

  uint16_t *dst = buf;
  for (const auto &id : ids) {
    *(dst++) = static_cast<uint16_t>(id);
  }
  assert(dst == buf + len);

  return;
}

void sentencepiece_processor_encode16_with_suffix(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, uint16_t suffix_tok, uint16_t **encoded, size_t *encoded_len) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  std::vector<int> ids = sentencepiece_processor->EncodeAsIds(sentence_view);

  size_t len = ids.size();
  *encoded_len = len + 1;

  uint16_t *buf = (uint16_t *)malloc(sizeof(uint16_t) * (len + 1));
  *encoded = buf;

  uint16_t *dst = buf;
  for (const auto &id : ids) {
    *(dst++) = static_cast<uint16_t>(id);
  }
  *(dst++) = suffix_tok;
  assert(dst == buf + len + 1);

  return;
}

void sentencepiece_processor_encode16_with_prefix(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, uint16_t prefix_tok, uint16_t **encoded, size_t *encoded_len) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  std::vector<int> ids = sentencepiece_processor->EncodeAsIds(sentence_view);

  size_t len = ids.size();
  *encoded_len = len + 1;

  uint16_t *buf = (uint16_t *)malloc(sizeof(uint16_t) * (len + 1));
  *encoded = buf;

  uint16_t *dst = buf;
  *(dst++) = prefix_tok;
  for (const auto &id : ids) {
    *(dst++) = static_cast<uint16_t>(id);
  }
  assert(dst == buf + len + 1);

  return;
}

void sentencepiece_processor_encode16_with_prefix_suffix(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, uint16_t prefix_tok, uint16_t suffix_tok, uint16_t **encoded, size_t *encoded_len) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  std::vector<int> ids = sentencepiece_processor->EncodeAsIds(sentence_view);

  size_t len = ids.size();
  *encoded_len = len + 2;

  uint16_t *buf = (uint16_t *)malloc(sizeof(uint16_t) * (len + 2));
  *encoded = buf;

  uint16_t *dst = buf;
  *(dst++) = prefix_tok;
  for (const auto &id : ids) {
    *(dst++) = static_cast<uint16_t>(id);
  }
  *(dst++) = suffix_tok;
  assert(dst == buf + len + 2);

  return;
}

} // extern "C"

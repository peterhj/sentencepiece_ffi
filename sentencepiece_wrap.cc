//#include <cstdint>
//#include <cstdlib>
//#include <cstring>
#include <type_traits>
#include <vector>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
//#include <ext/malloc_allocator.h>

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

/*int sentencepiece_processor_load(SentencePieceProcessor *sentencepiece_processor, char const *filename) {
  auto status = sentencepiece_processor->Load(filename);
  return to_underlying_type(status.code());
}*/

/*unsigned char *sentencepiece_processor_to_serialized_proto(SentencePieceProcessor *sentencepiece_processor, size_t *len) {
  auto serialized = sentencepiece_processor->serialized_model_proto();

  *len = serialized.size();
  unsigned char *data = (unsigned char *) malloc(serialized.size());
  memcpy(data, serialized.data(), serialized.size());

  return data;
}*/

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

    for (int const *piece = pieces; piece != pieces + pieces_len; ++piece) {
        int_pieces.push_back(static_cast<int>(*piece));
    }

    std::string decoded_string;
    auto status = sentencepiece_processor->Decode(int_pieces, &decoded_string);

    *decoded_len = decoded_string.size();
    *decoded = (char *)malloc(decoded_string.size());
    memcpy(*decoded, decoded_string.data(), decoded_string.size());

    return to_underlying_type(status.code());
}

/*int sentencepiece_processor_decode_pieces(SentencePieceProcessor *sentencepiece_processor, char const * const *pieces, size_t pieces_len, unsigned char **decoded, size_t *decoded_len) {
    std::vector<absl::string_view> str_pieces;
    str_pieces.reserve(pieces_len);
  
    for (char const * const *piece = pieces; piece != pieces + pieces_len; ++piece) {
        str_pieces.push_back(*piece);
    }

    std::string decoded_string;
    auto status = sentencepiece_processor->Decode(str_pieces, &decoded_string);

    *decoded_len = decoded_string.size();
    *decoded = (unsigned char *)malloc(decoded_string.size());
    memcpy(*decoded, decoded_string.data(), decoded_string.size());

    return to_underlying_type(status.code());
}*/

void sentencepiece_processor_encode(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, int **encoded, size_t *encoded_len) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  std::vector<int/*, __gnu_cxx::malloc_allocator<int>*/> ids = sentencepiece_processor->EncodeAsIds(sentence_view);

  // FIXME FIXME
  *encoded_len = ids.size();
  *encoded = (int *)malloc(sizeof(int) * ids.size());
  memcpy(*encoded, ids.data(), sizeof(int) * ids.size());

  return;
}

/*unsigned char *sentencepiece_processor_encode_as_serialized_proto(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, size_t *len) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  auto serialized = sentencepiece_processor->EncodeAsSerializedProto(sentence_view);

  *len = serialized.size();
  unsigned char *data = (unsigned char *) malloc(serialized.size());
  memcpy(data, serialized.data(), serialized.size());

  return data;
}

unsigned char *sentencepiece_processor_sample_encode_as_serialized_proto(SentencePieceProcessor *sentencepiece_processor, char const *sentence, size_t sentence_len, size_t *len, size_t nbest, float alpha) {
  auto sentence_view = absl::string_view(sentence, sentence_len);
  auto serialized = sentencepiece_processor->SampleEncodeAsSerializedProto(sentence_view, static_cast<int>(nbest), alpha);

  *len = serialized.size();
  unsigned char *data = (unsigned char *) malloc(serialized.size());
  memcpy(data, serialized.data(), serialized.size());

  return data;
}*/

} // extern "C"

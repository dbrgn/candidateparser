/**
 * C bindings for candidateparser library.
 * https://github.com/dbrgn/candidateparser
 **/

#ifndef candidateparser_bindings_h
#define candidateparser_bindings_h

/* Generated with cbindgen:0.1.23 */

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

// A key value pair.
typedef struct {
  const uint8_t *key;
  size_t key_len;
  const uint8_t *val;
  size_t val_len;
} KeyValuePair;

// A key value map.
// 
// The `len` must be set to the length of the `values` array. Everything else
// is undefined behavior!
typedef struct {
  const KeyValuePair *values;
  size_t len;
} KeyValueMap;

// A wrapper around the `IceCandidate` data that is C compatible.
typedef struct {
  const char *foundation;
  uint32_t component_id;
  const char *transport;
  uint64_t priority;
  const char *connection_address;
  uint16_t port;
  const char *candidate_type;
  // The address is optional. If no value is defined, this will contain a
  // null pointer.
  const char *rel_addr;
  // This port is optional. If no address is defined, this will contain the
  // value `0`.
  uint16_t rel_port;
  // The extensions map will always be defined but may be empty.
  KeyValueMap extensions;
} IceCandidateFFI;

// Free the memory associated with the [`IceCandidateFFI`](struct.IceCandidateFFI.html) struct.
// 
// Make sure to always call this function after you're done processing the
// data, otherwise you'll end up with memory leaks!
void free_ice_candidate(const IceCandidateFFI *ptr);

// Parse an ICE candidate SDP string and return a pointer to an
// [`IceCandidateFFI`](struct.IceCandidateFFI.html) struct.
// 
// Make sure to always call the [`free_ice_candidate`](fn.free_ice_candidate.html)
// function after you're done processing the data, to prevent memory leaks!
const IceCandidateFFI *parse_ice_candidate_sdp(const char *sdp);

#endif // candidateparser_bindings_h

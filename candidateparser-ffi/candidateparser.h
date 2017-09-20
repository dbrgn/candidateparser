/**
 * C bindings for candidateparser library.
 * (c) 2017 Danilo Bargen
 **/

#ifndef candidateparser_bindings_h
#define candidateparser_bindings_h

/* Generated with cbindgen:0.1.23 */

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

// A wrapper around the IceCandidate type that is C compatible.
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
} IceCandidateFFI;

const IceCandidateFFI *parse_ice_candidate_sdp(const char *sdp);

#endif // candidateparser_bindings_h

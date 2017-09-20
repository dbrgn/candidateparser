/**
 * C example.
 *
 * Compile like this:
 *
 *   $ clang parse.c -o parse -L ../target/debug -l candidateparser_ffi -Wall -Wextra -g
 *
 * Run like this:
 *
 *   $ LD_LIBRARY_PATH=../target/debug/ ./parse
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#include "candidateparser.h"

int main() {
    const char *sdp = "candidate:842163049 1 udp 1686052607 1.2.3.4 46154 typ srflx raddr 10.0.0.17 rport 1337 generation 0 ufrag EEtu network-id 3 network-cost 10";

    printf("Parsing candidate:\n\n  %s\n", sdp);
    const IceCandidateFFI *candidate = parse_ice_candidate_sdp(sdp);

    printf("\nResults:\n\n");
    printf("  Foundation:    %s\n", candidate->foundation);
    printf("  Component ID:  %u\n", candidate->component_id);
    printf("  Transport:     %s\n", candidate->transport);
    printf("  Priority:      %lu\n", candidate->priority);
    printf("  Address:       %s\n", candidate->connection_address);
    printf("  Port:          %hu\n", candidate->port);
    printf("  Type:          %s\n", candidate->candidate_type);
    printf("  Rel Addr:      %s\n", candidate->rel_addr);
    printf("  Rel Port:      %hu\n", candidate->rel_port);
    printf("\nBrought to you by the powers of Rust!\n");
    return 0;
}

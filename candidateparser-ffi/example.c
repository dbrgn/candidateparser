/**
 * C example.
 *
 * Compile like this:
 *
 *   $ clang example.c -o example -L ../target/debug -l candidateparser_ffi -Wall -Wextra -g
 *
 * Run like this:
 *
 *   $ LD_LIBRARY_PATH=../target/debug/ ./example
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

#include "candidateparser.h"

/**
 * Print the character if it's printable, print a "?" mark otherwise.
 */
static inline void print_byte(uint8_t val) {
    if (val < 0x20 || val > 0x7e) { // Non printable
        printf("?");
    } else { // Printable
        printf("%c", val);
    }
}

/**
 * Print `len` number of bytes from `values` to stdout.
 */
static inline void print_bytes(uint8_t const *values, size_t len) {
    for (size_t i = 0; i < len; i++) {
        print_byte(values[i]);
    }
}

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
    if (candidate->extensions.len <= 0) {
        printf("  Extensions:    -\n");
    } else {
        printf("  Extensions:\n");
        for (size_t i = 0; i < candidate->extensions.len; i++) {
            printf("    - ");
            print_bytes(candidate->extensions.values[i].key,
                        candidate->extensions.values[i].key_len);
            printf(" => ");
            print_bytes(candidate->extensions.values[i].val,
                        candidate->extensions.values[i].val_len);
            printf("\n");
        }
    }

    printf("\nCleaning up memory resources... ");
    free_ice_candidate(candidate);
    printf("\n\nBrought to you by the powers of Rust!\n");
    return 0;
}

/**
 * C++ example. Based on Danilo Bargens original C example.
 *
 * Compile like this:
 *
 *   $ clang++ example.cpp -o example -L ../target/debug -l candidateparser_ffi -Wall -Wextra -g
 *
 * Or use:
 *
 *   $ make examples/ffi/cpp
 *
 * Run like this:
 *
 *   $ LD_LIBRARY_PATH=../target/debug/ ./example
 */

#include "candidateparser.hpp"
#include <iostream>

int main()
  {
  auto constexpr descriptor = "candidate:842163049 "
                              "1 "
                              "udp "
                              "1686052607 "
                              "1.2.3.4 46154 "
                              "typ srflx "
                              "raddr 10.0.0.17 "
                              "rport 1337 "
                              "generation 0 "
                              "ufrag EEtu "
                              "network-id 3 "
                              "network-cost 10";

  auto const candidate = dbrgn::IceCandidate::parse(descriptor);

  std::cout << (candidate.type == dbrgn::kCandidateTypeSrflx ? "Candidate is SRFLX" : "Candidate is not SRFLX")
            << "\nCandidate Descriptor Dump:\n"
            << candidate
            << '\n';
  }

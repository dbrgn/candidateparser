/**
 * Copyright (c) 2017 Danilo Bargen and contributors
 *
 * Dual-licensed under MIT and Apache licenses.
 * See the license files in the source root for more information.
 *
 * Contributors:
 * - Felix Morgner <felix.morgner@gmail.com> - Initial API and implementation
 */

#ifndef CANDIDATEPARSER_HPP
#define CANDIDATEPARSER_HPP

#if __cplusplus < 201703L
#error "Requires C++17 or above"
#endif

#include <cctype>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <ostream>
#include <string>
#include <string_view>
#include <utility>

extern "C" {
#include "candidateparser.h"
}

namespace dbrgn
  {

  namespace internal
    {
    auto constexpr make_view(char const * const string)
      {
      auto length = 0ull;
      while(string[length] != '\0')
        {
        ++length;
        }

      return std::string_view{string, length};
      }

    template<typename Map, typename Key = typename Map::key_type, typename Value = typename Map::mapped_type>
    auto incarnate_map(::KeyValueMap const & data)
      {
      auto && map = Map{};

      for(auto pairIdx = 0ull; pairIdx < data.len; ++pairIdx)
        {
        auto && rustPair = data.values[pairIdx];
        map.emplace(std::make_pair(Key{rustPair.key, rustPair.key_len}, Value{rustPair.val, rustPair.val_len}));
        }

      return map;
      }

    template<typename ByteContainer>
    auto write_bytes(std::ostream & out, ByteContainer container)
      {
      for(auto const & byte : container)
        {
        if(std::isprint(byte))
          {
          out << byte;
          }
        else
          {
          out << '?';
          }
        }
      }
    }

  constexpr struct Transport
    {
    std::string_view const value;

    friend std::ostream & operator<<(std::ostream & out, Transport const & tranport)
      {
      return out << tranport.value;
      }

    bool operator==(Transport const & other) const { return value == other.value; };

    } kTransportUdp{internal::make_view("udp")};

  constexpr struct CandidateType
    {
    std::string_view const value;

    friend std::ostream & operator<<(std::ostream & out, CandidateType const & type)
      {
      return out << type.value;
      }

    bool operator==(CandidateType const & other) const { return value == other.value; };

    } kCandidateTypeHost{internal::make_view("host")},
      kCandidateTypeSrflx{internal::make_view("srflx")},
      kCandidateTypePrfls{internal::make_view("prflx")},
      kCandidateTypeRelay{internal::make_view("relay")};

  class IceCandidate
    {

    explicit IceCandidate(::IceCandidateFFI const * const rust_data)
      : m_rustData{rust_data, ::free_ice_candidate}
      , foundation{m_rustData->foundation}
      , component_id{m_rustData->component_id}
      , transport{m_rustData->transport}
      , priority{m_rustData->priority}
      , connection_address{m_rustData->connection_address}
      , port{m_rustData->port}
      , type{m_rustData->candidate_type}
      , rel_address{m_rustData->rel_addr}
      , rel_port{m_rustData->rel_port}
      , extensions{internal::incarnate_map<decltype(extensions)::value_type>(m_rustData->extensions)}
      { }

    std::unique_ptr<::IceCandidateFFI const, void (*)(::IceCandidateFFI const *)> m_rustData;

    public:

      static inline auto parse(std::string const & data)
        {
        return IceCandidate{::parse_ice_candidate_sdp(data.c_str())};
        }

      friend std::ostream & operator<<(std::ostream & out, IceCandidate const & candidate)
        {
        out << "IceCandidate : {\n"
            << "\tfoundation         : " << candidate.foundation << '\n'
            << "\tcomponent_id       : " << candidate.component_id << '\n'
            << "\ttransport          : " << candidate.transport << '\n'
            << "\tpriority           : " << candidate.priority << '\n'
            << "\tconnection_address : " << candidate.connection_address << '\n'
            << "\tport               : " << candidate.port << '\n'
            << "\ttype               : " << candidate.type << '\n'
            << "\trel_address        : " << (candidate.rel_address.has_value() ? candidate.rel_address.value() : "") << '\n'
            << "\trel_port           : " << (candidate.rel_port.has_value() ? candidate.rel_port.value() : '\0') << '\n'
            ;

        out << "\textensions         : ";
        if(candidate.extensions.has_value())
          {
          out << "{\n";
          auto && extensions = candidate.extensions.value();
          for(auto const & extension : extensions)
            {
            out << "\t\t";
            internal::write_bytes(out, extension.first);
            out << " => ";
            internal::write_bytes(out, extension.second);
            out << '\n';
            }
          }
        else
          {
          out << "-\n";
          }

        out << '}';
        return out;
        }

      std::string_view const foundation;
      std::uint32_t const component_id;
      Transport const transport;
      std::uint64_t const priority;
      std::string_view const connection_address;
      std::uint16_t const port;
      CandidateType const type;
      std::optional<std::string_view> const rel_address;
      std::optional<std::uint16_t> const rel_port;
      std::optional<std::map<std::basic_string_view<std::uint8_t>, std::basic_string_view<std::uint8_t>>> const extensions;
    };

  }

#endif

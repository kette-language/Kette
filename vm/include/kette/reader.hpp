#pragma once

#include <charconv>
#include <kette/defaults.hpp>

namespace kette {
  using NumberWord = std::variant<u64, i64, f64>;

  struct StringWord {
    std::string value;
  };
  struct IdentifierWord {
    std::string value;
  };

  using WordData = std::variant<std::monostate, IdentifierWord, StringWord, NumberWord>;

  struct RawWord {
    std::string_view value;
    i64 line, column;
  };
  
  struct Word {
    WordData data;
    i64 line, column;
  
    Word();
    Word(WordData word, const RawWord& raw);
    auto isNull() -> bool;
  };



  class Reader {
  private:
    std::string_view source;
    i64 cursor;
    i64 line, column;

  public:
    Reader(std::string_view source);

  private:
    auto incrementLine();
    auto incrementColumn();
    auto skipWhitespace();

  public:
    auto readWordRaw() -> std::optional<RawWord>;
    auto readWord() -> Word;
  };

  auto to_string(const Word &word) -> std::string;

  template <typename T> auto try_parse(std::string_view str) -> std::optional<T> {
    T parsed_number{};
    auto [ptr, error_code] = std::from_chars(str.data(), str.data() + str.size(), parsed_number);

    if (error_code == std::errc{}) {
      if (std::is_integral<T>::value && (ptr != str.data() + str.size() && (*ptr == '.' || !std::isdigit(*ptr)))) {
        return std::nullopt;
      }
      return parsed_number;
    } else {
      return std::nullopt;
    }
  }

} // namespace kette

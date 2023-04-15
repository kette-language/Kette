#include <string_view>
#include <string>
#include <variant>
#include <optional>
#include <kette/reader.hpp>
#include <kette/defaults.hpp>
#include <iostream>

namespace kette {
  Reader::Reader(std::string_view a_source) {
    source = a_source;
    cursor = 0;
    line   = 1;
    column = 1;
  }

  auto Reader::increment_line() {
    cursor++;
    line++;
    column = 1;
  }

  auto Reader::increment_column() {
    cursor++;
    column++;
  }

  auto Reader::skip_whitespace() {
    auto sub = source.substr(cursor);
    for (auto const &c : sub) {
      if (c == ' ' || c == '\t')
        increment_column();
      else if (c == '\n')
        increment_line();
      else
        break;
    }
  }

  auto Reader::read_word_raw() -> std::optional<RawWord> {
    skip_whitespace();
    if (source.length() == cursor) return std::nullopt;

    auto start_line   = line;
    auto start_column = column;
    auto start_cursor = cursor;
    auto sub          = source.substr(cursor);
    auto is_string    = false;
    for (auto const &c : sub) {
      if (!is_string && (c == ' ' || c == '\t' || c == '\n')) break;
      if (c == '\"') {
        if (is_string) {
          increment_column();
          break;
        } else {
          is_string = true;
        }
      }
      increment_column();
    }
    auto length = cursor - start_cursor;
    auto raw    = source.substr(start_cursor, length);
    return RawWord{raw, start_line, start_column};
    ;
  }

  auto Reader::read_word() -> Word {
    auto maybe_raw = read_word_raw();
    if (!maybe_raw.has_value()) return Word{};
    auto raw = maybe_raw.value();

    // string
    if (raw.value.starts_with("\""))
      return StringWord{std::string{raw.value.substr(1, raw.value.length() - 2)}};

    // number
    if (auto val = try_parse<u64>(raw.value)) return NumberWord{*val};
    if (auto val = try_parse<i64>(raw.value)) return NumberWord{*val};
    if (auto val = try_parse<f64>(raw.value)) return NumberWord{*val};

    return IdentifierWord{std::string{raw.value}};
  }

  auto to_string(const Word &word) -> std::string {
    if (auto val = std::get_if<IdentifierWord>(&word)) return val->value;
    if (auto val = std::get_if<StringWord>(&word)) return val->value;
    if (auto val = std::get_if<NumberWord>(&word))
      return std::visit(
          [](auto &&num) -> auto{ return std::to_string(num); }, *val);
    else
      return "None";
  }
} // namespace kette

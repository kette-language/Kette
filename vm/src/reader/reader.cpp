#include <string_view>
#include <string>
#include <variant>
#include <optional>
#include <kette/reader.hpp>
#include <kette/defaults.hpp>
#include <iostream>

namespace kette {
  Word::Word() {
    data = std::monostate();
    line = -1;
    column = -1;
  }

  Word::Word(WordData word, const RawWord& raw) {
    data = word;
    line = raw.line;
    column = raw.column;
  }

  auto Word::isNull() -> bool {
    return std::holds_alternative<std::monostate>(data);
  }
  
  Reader::Reader(std::string_view a_source) {
    source = a_source;
    cursor = 0;
    line   = 1;
    column = 1;
  }

  auto Reader::incrementLine() {
    cursor++;
    line++;
    column = 1;
  }

  auto Reader::incrementColumn() {
    cursor++;
    column++;
  }

  auto Reader::skipWhitespace() {
    auto sub = source.substr(cursor);
    for (auto const &c : sub) {
      if (c == ' ' || c == '\t')
        incrementColumn();
      else if (c == '\n')
        incrementLine();
      else
        break;
    }
  }

  auto Reader::readWordRaw() -> std::optional<RawWord> {
    skipWhitespace();
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
          incrementColumn();
          break;
        } else {
          is_string = true;
        }
      }
      incrementColumn();
    }
    auto length = cursor - start_cursor;
    auto raw    = source.substr(start_cursor, length);
    return RawWord{raw, start_line, start_column};
    ;
  }

  auto Reader::readWord() -> Word {
    auto maybe_raw = readWordRaw();
    if (!maybe_raw.has_value()) return Word();
    auto raw = maybe_raw.value();

    // string
    if (raw.value.starts_with("\""))
      return Word(StringWord{ std::string(raw.value.substr(1, raw.value.length() - 2)) }, raw);

    // number
    if (auto val = try_parse<u64>(raw.value)) return Word(NumberWord(*val), raw);
    if (auto val = try_parse<i64>(raw.value)) return Word(NumberWord(*val), raw);
    if (auto val = try_parse<f64>(raw.value)) return Word(NumberWord(*val), raw);

    return Word(IdentifierWord { std::string(raw.value) }, raw);
  }

  auto to_string(const Word &word) -> std::string {
    return std::visit(match {
      [](IdentifierWord const& val) { return val.value; },
      [](StringWord const& val) { return val.value; },
      [](NumberWord const& val) { 
        return std::visit(
          [](auto &&num) { return std::to_string(num); }, val);
        },
      [](auto) { return std::string{"None"}; },
    }, word.data);
  }
} // namespace kette

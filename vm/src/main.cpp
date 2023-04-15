#include <kette/reader.hpp>
#include <variant>
#include <iostream>

auto main() -> i32 {
  auto reader = kette::Reader(R"(
    5 3 + . 6.420 3.0 - .
  )");

  for(auto word = reader.read_word(); !std::holds_alternative<std::monostate>(word); word = reader.read_word()) {
    auto str = kette::to_string(word);
    std::cout << "\"" << str << "\"" << ", ";
  }
  std::cout << std::endl;
  return 0;
}

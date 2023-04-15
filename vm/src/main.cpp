#include <kette/reader.hpp>
#include <variant>
#include <iostream>

auto main() -> i32 {
  auto reader = kette::Reader(R"(
    5 3 + . 6.420 3.0 - . "lol" println
  )");

  for (auto word = reader.read_word(); !std::holds_alternative<std::monostate>(word); word = reader.read_word()) {
    std::cout << "\'" << kette::to_string(word) << "\'"
              << ", ";
  }
  std::cout << std::endl;
  return 0;
}

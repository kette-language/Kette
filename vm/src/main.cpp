#include <kette/reader.hpp>
#include <kette/mem.hpp>
#include <variant>
#include <iostream>

const auto DEMO = R"(
    5 3 + . 6.420 3.0 - . "lol" println
)";

auto main() -> i32 {
  auto reader = kette::Reader(DEMO);

  for (auto word = reader.read_word(); !std::holds_alternative<std::monostate>(word); word = reader.read_word()) {
    std::cout << "\'" << kette::to_string(word) << "\'"
              << ", ";
  }

  auto mem = kette::mem::ExecutableMemory { 200 }; 
  mem.resize(7000);
  std::cout << std::endl;
  return 0;
}

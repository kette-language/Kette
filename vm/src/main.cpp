#include <kette/reader.hpp>
#include <kette/mem.hpp>
#include <kette/defaults.hpp>

const auto DEMO = R"(
    5 3 + . 6.420 3.0 - . "lol" println
)";

auto main() -> i32 {
  auto reader = kette::Reader(DEMO);

  for (auto word = reader.read_word(); !std::holds_alternative<std::monostate>(word); word = reader.read_word()) {
    std::cout << "\'" << kette::to_string(word) << "\'"
              << ", ";
  }
  std::cout << std::endl;

  auto mem = kette::mem::ExecutableMemory { 1300 }; 
  mem.push(0xC3);

  auto fn = mem.to_function();
  fn();
  std::cout << mem.to_string() << std::endl;
  return 0;
}

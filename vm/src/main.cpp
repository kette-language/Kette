#include <kette/reader.hpp>

auto main() -> i32 {
  auto reader = Kette::Reader(R"(
    5 3 + drop
  )");
  return 0;
}

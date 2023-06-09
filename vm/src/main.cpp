#include <kette/context.hpp>
#include <kette/mem.hpp>
#include <kette/defaults.hpp>
#include <kette/objects.hpp>

const auto DEMO = R"(
  5 -3 + . 6.420 3.0 - .
)";

auto main() -> i32 {
  // auto context = kette::Context();
  // context.addBuiltins();
  // context.execute(DEMO);

  // auto mem = kette::mem::ExecutableMemory { 1300 }; 
  // mem.push(0xC3);

  // auto fn = mem.to_function();
  // fn();
  // std::cout << mem.to_string() << std::endl;

  kette::oo::tests::all();
  return 0;
}

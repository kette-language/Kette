#include <kette/builtins.hpp>

namespace kette::builtins {
  auto print_num(cell num) -> void {
    printf("%lu\n", num);
  }
}
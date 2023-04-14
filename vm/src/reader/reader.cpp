#include <string_view>
#include <kette/reader.hpp>
#include <kette/types.hpp>

namespace Kette {
  Reader::Reader(std::string_view a_source) {
    source = a_source;
    cursor = 0;
    line   = 0;
    column = 0;
  }
} // namespace Kette

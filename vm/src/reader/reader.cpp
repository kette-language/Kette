#include <string_view>
#include <kette/reader.h>
#include <kette/types.h>

namespace Kette {
  Reader::Reader(std::string_view source) {
    this->source = source;
    this->cursor = 0;
    this->line = 0;
    this->column = 0;
  }


}
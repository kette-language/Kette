#include <string_view>
#include <kette/types.h>

namespace Kette {

class Reader {
  private:
    std::string_view source;
    i64 cursor;
    i64 line, column;

  public:
    Reader(std::string_view source);
};

}
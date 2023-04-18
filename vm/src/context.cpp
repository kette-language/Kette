#include <kette/defaults.hpp>
#include <kette/context.hpp>
#include <kette/reader.hpp>

namespace kette {
  Context::Context() {
    symbols = new SymbolTable;
  }

  Context::~Context() {
    delete symbols;
  }

  auto Context::execute(std::string_view str) -> void {
    auto reader = Reader(str);
    for (auto word = reader.readWord(); !word.isNull(); word = reader.readWord()) {
      std::visit(match {
        [&](IdentifierWord const& val) { 
          if (val.value == "macro:") {
            printf("todo: Macros\n");
            return;
          }
          if (symbols->contains(val.value)) {
            printf("todo: Symbol Lookup\n");
          } else {
            printf("todo: Register Lookup after compilation unit\n");
          }
        },
        [](StringWord const& val) { 
          printf("todo: Strings\n");
          return;
        },
        [](NumberWord const& val) { 
          std::visit([](auto &&num) {
            printf("todo: Numbers\n");
            return;; 
          }, val);
        },
        [](auto) { },
      }, word.data);
    }
  }
} // namespace kette
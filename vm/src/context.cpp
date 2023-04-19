#include <kette/context.hpp>
#include <kette/reader.hpp>

namespace kette {
  Context::Context() {
    symbols = new SymbolTable;
    cfgs = new CFGS;
  }

  Context::~Context() {
    delete symbols;
    delete cfgs;
  }

  auto Context::execute(std::string_view str) -> void {
    auto reader = Reader(str);
    
    for (auto word = reader.readWord(); !word.isNull(); word = reader.readWord()) {
      std::visit(match {
        [&](IdentifierWord const& val) { 
          if (val.value == "macro:") {
            printf("todo: define reader macro\n");
            return;
          }
          if (symbols->contains(val.value)) {
            auto sym = symbols->get(val.value);
            if (sym->kind == ReaderMacro) {
              printf("todo: execute reader macro");
            }
          } else {
            printf("todo: Unknown Symbol => Lookup after compilation unit\n");
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

  auto Context::addBuiltinSymbols() -> void {
    symbols->insert(Symbol { "+", SymbolKind::Builtin });
    symbols->insert(Symbol { "-", SymbolKind::Builtin });
    symbols->insert(Symbol { "*", SymbolKind::Builtin });
    symbols->insert(Symbol { "/", SymbolKind::Builtin });
    symbols->insert(Symbol { ".", SymbolKind::Builtin });
    symbols->insert(Symbol { "drop", SymbolKind::Builtin });
    symbols->insert(Symbol { "dup", SymbolKind::Builtin });
    symbols->insert(Symbol { "2dup", SymbolKind::Builtin });
    symbols->insert(Symbol { "swap", SymbolKind::Builtin });
    symbols->insert(Symbol { "2swap", SymbolKind::Builtin });
    symbols->insert(Symbol { "over", SymbolKind::Builtin });
    symbols->insert(Symbol { "2over", SymbolKind::Builtin });
    symbols->insert(Symbol { "rot", SymbolKind::Builtin });
    symbols->insert(Symbol { "2rot", SymbolKind::Builtin });
    symbols->insert(Symbol { "pick", SymbolKind::Builtin });
    symbols->insert(Symbol { "2pick", SymbolKind::Builtin });
    symbols->insert(Symbol { "dip", SymbolKind::Builtin });
    
    auto id_colon = symbols->insert(Symbol { ":", SymbolKind::ReaderMacro });
    auto id_string = symbols->insert(Symbol { "\"", SymbolKind::ReaderMacro });
    auto id_backslach = symbols->insert(Symbol { "\\", SymbolKind::ReaderMacro});
    // todo: write reader macros in C++
  }
} // namespace kette
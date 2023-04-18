#include <kette/defaults.hpp>
#include <map>

namespace kette::symbols {
  using SymbolId = usize;

  struct Symbol {
    std::string identifier;
    bool builtin;
  };

  class SymbolTable {
  private:
    std::map<std::string, SymbolId> mappings;
    std::map<SymbolId, Symbol> symbols;
    SymbolId nextId;

  private:
    auto getNextId() -> SymbolId;

  public:
    auto insert(Symbol sym) -> SymbolId;
  };
}
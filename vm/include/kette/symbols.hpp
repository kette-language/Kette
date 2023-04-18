#include <kette/defaults.hpp>
#include <map>

namespace kette {
  using SymbolId = usize;

  enum SymbolKind {
    Builtin,
    ReaderMacro,
    ReaderMacroInstance,
  };

  struct Symbol {
    std::string identifier;
    SymbolKind kind;
  };

  class SymbolTable {
  private:
    std::map<std::string, SymbolId> mappings;
    std::map<SymbolId, Symbol> symbols;
    SymbolId nextId;

  private:
    auto getNextId() -> SymbolId;

  public:
    auto contains(std::string_view str) -> bool;
    auto insert(Symbol sym) -> SymbolId;
    auto getId(std::string_view str) -> std::optional<SymbolId>;
    auto getFromId(SymbolId id) -> Symbol*;
    auto get(std::string_view str) -> Symbol*;
  };
}
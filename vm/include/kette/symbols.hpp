#pragma once

#include <kette/defaults.hpp>
#include <map>

namespace kette {
  class StringPool {
  private:
    std::map<std::string, cell> strings;
    cell next_id;
  public:
    auto get_next_id() -> cell;
    auto insert_string(const std::string& str) -> cell;
    auto string(const std::string& str) -> cell;
    auto get_string(cell id) const -> std::string;
  };

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
    std::map<SymbolId, SymbolId> reader_macros;
    SymbolId nextId;

  private:
    auto getNextId() -> SymbolId;

  public:
    auto contains(std::string_view str) -> bool;
    auto insert(Symbol sym) -> SymbolId;
    auto insertReaderMacro(SymbolId macro, SymbolId instance);
    auto getId(std::string_view str) -> std::optional<SymbolId>;
    auto getFromId(SymbolId id) -> Symbol*;
    auto get(std::string_view str) -> std::tuple<SymbolId, Symbol*>;
  };
}
#include <kette/symbols.hpp>

namespace kette {
  auto SymbolTable::getNextId() -> SymbolId {
    auto id = nextId;
    nextId++;
    return id;
  }

  auto SymbolTable::contains(std::string_view str) -> bool {
    return mappings.contains(std::string(str));
  }
  
  auto SymbolTable::insert(Symbol sym) -> SymbolId {
    auto id = getNextId();
    mappings[sym.identifier] = id;
    symbols[id] = sym;
    return id;
  }

  auto SymbolTable::getId(std::string_view str) -> std::optional<SymbolId> {
    auto search = std::string(str);
    if (!mappings.contains(search)) return {};
    return mappings[std::string(str)];
  }

  auto SymbolTable::getFromId(SymbolId id) -> Symbol* {
    if (!symbols.contains(id)) return nullptr;
    return &symbols[id];
  }

  auto SymbolTable::get(std::string_view str) -> Symbol* {
    auto key = getId(str);
    if (!key.has_value()) return nullptr;
    return &symbols[key.value()];
  }
}
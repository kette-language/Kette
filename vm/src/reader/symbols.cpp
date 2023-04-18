#include <kette/symbols.hpp>

namespace kette::symbols {
  auto SymbolTable::getNextId() -> SymbolId {
    auto id = nextId;
    nextId++;
    return nextId;
  }
  
  auto SymbolTable::insert(Symbol sym) -> SymbolId {
    auto id = getNextId();
    mappings[sym.identifier] = id;
    symbols[id] = sym;
    return id;
  }
}
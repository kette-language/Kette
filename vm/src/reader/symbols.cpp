#include <kette/symbols.hpp>

namespace kette {
  auto StringPool::get_next_id() -> cell {
    auto id = next_id;
    next_id++;
    return id;
  }

  auto StringPool::insert_string(const std::string& str) -> cell {
    auto id = get_next_id();
    strings[str] = id;
    return id;    
  }

  auto StringPool::get_string(cell id) const -> std::string {
    for (const auto& kv : strings) {
      if(kv.second == id) return kv.first;
    }
    return "";
  }

  auto StringPool::string(const std::string& str) -> cell {
    if (strings.contains(str)) return strings[str];
    return insert_string(str);
  }


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

  auto SymbolTable::get(std::string_view str) -> std::tuple<SymbolId, Symbol*> {
    auto key = getId(str);
    if (!key.has_value()) return { -1, nullptr } ;
    return {key.value(), &symbols[key.value()]};
  }
}
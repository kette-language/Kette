#pragma once
#include <kette/defaults.hpp>

namespace kette::oo {

  constexpr uintptr_t TAG_MASK = 0x3;
  
  enum TagValues {
    Integer = 0x1,
    Float = 0x2,
};

  // Utility functions to add, extract, and remove tags
  template <typename T>
  T* add_tag(T* ptr, uintptr_t tag) {
    return reinterpret_cast<T*>(reinterpret_cast<uintptr_t>(ptr) | (tag & TAG_MASK));
  }

  template <typename T>
  uintptr_t extract_tag(T* ptr) {
    return reinterpret_cast<uintptr_t>(ptr) & TAG_MASK;
  }

  template <typename T>
  T* remove_tag(T* ptr) {
    return reinterpret_cast<T*>(reinterpret_cast<uintptr_t>(ptr) & ~TAG_MASK);
  }

  // only the predfined ones, futher can be defined.
  // the ones defined here must be kept in Sync!
  enum SlotTypes {
    none,
    parent,
    value,
    accessor,
    function,
    method,
  };

  struct SlotDesc {
    cell name;
    cell type; // SlotTypes
    cell offset;
  };

  struct Map {
    cell id;
    cell name;
    cell allocator;
    cell slot_count;
    SlotDesc* slot_descs;
  };

  struct Object {
    cell* map;
    cell slot_count;
    cell* slots;

    Object(Map* mapp);
    Object(cell* mapp);
    auto get_property(cell name) -> cell;
    auto set_property(cell name, cell value) -> void;
    auto clone() -> Object;
    auto instanciate(cell* map_table) -> Object; // like clone but only
  };

  class MapTable {
  private:
    std::map<std::string, cell> strings;
    std::map<cell, Map*> maps;
    cell next_string_id;
    cell next_map_id;
  
  public:
    auto get_next_string_id() -> cell;
    auto get_next_map_id() -> cell;
    auto insert_string(const std::string& str) -> cell;
    auto get_or_insert_string(const std::string& str) -> cell;
    auto get_string(cell id) const -> std::string;
    auto create_map(cell name, SlotDesc* descs, cell slot_count, cell allocator) -> Map*;
    auto get_map(cell id) -> Map*;
    auto get_map(const std::string& str) -> Map*;

    inline auto string(const std::string& str) -> cell {
      return get_or_insert_string(str);
    }
  };

  auto to_string(const Object& obj, MapTable& maps, int level = 0) -> std::string;
  namespace tests {
    auto all() -> bool;
  }
}
#pragma once
#include <kette/defaults.hpp>

namespace kette::oo {
  #define MAP_MAP cell { 0 };

  // only the predfined ones, futher can be defined.
  // the ones defined here must be kept in Sync!
  enum SlotTypes {
    none,
    parent,
    value,
  };

  struct SlotDesc {
    cell name;
    cell type; // SlotTypes
    cell offset;
  };

  struct Map {
    cell id;
    cell name;
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
    auto create_map(cell name, SlotDesc* descs, cell slot_count) -> Map*;
  };

  auto to_string(const Object& obj, MapTable& maps, int level = 0) -> std::string;
  namespace tests {
    auto all() -> bool;
  }
}
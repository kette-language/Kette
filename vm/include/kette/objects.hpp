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
    cell type;
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
    auto get_property(cell name) -> cell;
    auto set_property(cell name, cell value) -> void;
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
    auto get_string(cell id) -> std::string_view;
    auto create_map(cell name, SlotDesc* descs, cell slot_count) -> Map*;
  };

  namespace tests {

  }
}
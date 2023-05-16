#pragma once
#include <kette/defaults.hpp>

namespace kette::oo {
  // only the predfined ones, futher can be defined.
  // the ones defined here must be kept in Sync!
  enum SlotTypes {
    Value,
    Parent,
    MapParent,
    Function,
  };

  struct SlotDesc {
    cell name;
    cell type; // SlotTypes
    cell offset;
  };

  struct Map {
    cell mark;
    cell id;
    cell name;
    cell value_slot_count;
    cell slot_count;
    SlotDesc* slot_descs;
  };

  struct Object {
    cell mark; // multipurpose field, can be used for GC, Ref count, atomics etc.
    cell* map;
    cell* slots;
  };

  auto new_object(cell* map) -> Object;
  auto get_property(Object* obj, cell name) -> cell; // -1 if not found
  auto set_property(Object* obj, cell name, cell value) -> cell; // -1 if not found
  auto clone(Object* other) -> Object;
  auto instantiate(Object* proto) -> Object;

  auto new_integer_object(i64 value, cell* map) -> Object;
  auto new_float_object(f64 value, cell* map) -> Object;

  class MapTable {
  private:
    std::map<cell, Map*> maps;
    cell next_map_id;
  public:
    auto get_next_map_id() -> cell;
    auto create_map(cell name, SlotDesc* descs, cell slots, cell value_slots) -> Map*;
    auto get_map(cell id) -> Map*;
    auto get_map(const std::string& str) -> Map*;
  };

  auto to_string(const Object& obj, MapTable& maps, int level = 0) -> std::string;
  namespace tests {
    auto all() -> bool;
  }
}
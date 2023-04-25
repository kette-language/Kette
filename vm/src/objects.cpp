#include <kette/objects.hpp>
#include <cassert>

namespace kette::oo {

  auto MapTable::get_next_string_id() -> cell {
    auto id = next_string_id;
    next_string_id++;
    return id;
  }

  auto MapTable::get_next_map_id() -> cell {
    auto id = next_map_id;
    next_map_id++;
    return id;
  }

  auto MapTable::insert_string(const std::string& str) -> cell {
    auto id = get_next_string_id();
    strings[str] = id;
    return id;    
  }

  auto MapTable::get_string(cell id) -> std::string_view {
    for (const auto [key, value] : strings) {
      if(value == id) return std::string_view(key);
    }
    assert(false && "this shouldn't happen lmao");
  }

  auto MapTable::get_or_insert_string(const std::string& str) -> cell {
    if (strings.contains(str)) return strings[str];
    return insert_string(str);
  }

  auto MapTable::create_map(cell name, SlotDesc* slot_descs, cell slot_count) -> Map* {
    auto id = get_next_map_id();
    auto map = new Map;
    map->id = id;
    map->name = name;
    map->slot_count = slot_count;
    map->slot_descs = new SlotDesc[slot_count];
    for (auto i = 0; i < slot_count; i++) {
      map->slot_descs[i] = slot_descs[i];
    }
    maps[id] = map;
    return map;
  }

  Object::Object(Map* mapp) {
    map = reinterpret_cast<cell*>(mapp);
    slot_count = mapp->slot_count;
    slots = new cell[slot_count];
  }

  auto Object::get_property(cell name) -> cell {
    Map* mapp = reinterpret_cast<Map*>(map);
    for (auto i = 0; i < mapp->slot_count; i++) {
      if (mapp->slot_descs[i].name == name)
        return slots[mapp->slot_descs[i].offset];
    }
    for (auto i = 0; i < mapp->slot_count; i++) {
      if (mapp->slot_descs[i].name == name) {
        if (mapp->slot_descs[i].type == SlotTypes::parent) {
          auto parent = reinterpret_cast<Object*>(slots[mapp->slot_descs[i].offset]);
          auto prop = parent->get_property(name);
          if (prop != -1) return prop;
        }
      }
    }
    return -1;
  }

  auto Object::set_property(cell name, cell value) -> void {
    Map* mapp = reinterpret_cast<Map*>(map);
    for (size_t i = 0; i < mapp->slot_count; ++i) {
      if (mapp->slot_descs[i].name == name) {
        slots[mapp->slot_descs[i].offset] = value;
        return;
      }
    }
    assert(false && "Property not found");
  }

}
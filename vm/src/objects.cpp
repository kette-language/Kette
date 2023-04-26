#include <kette/objects.hpp>
#include <cassert>
#include <cstring>

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

  auto MapTable::get_string(cell id) const -> std::string {
    for (const auto& kv : strings) {
      if(kv.second == id) return kv.first;
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

  Object::Object(cell* mapp) {
    auto mappp = reinterpret_cast<Map*>(mapp);
    map = mapp;
    slot_count = mappp->slot_count;
    slots = new cell[slot_count];
  }

  auto Object::get_property(cell name) -> cell {
    auto mapp = reinterpret_cast<Map*>(map);
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
    auto mapp = reinterpret_cast<Map*>(map);
    for (size_t i = 0; i < mapp->slot_count; ++i) {
      if (mapp->slot_descs[i].name == name) {
        slots[mapp->slot_descs[i].offset] = value;
        return;
      }
    }
    assert(false && "Property not found");
  }

  auto Object::clone() -> Object {
    auto obj = Object(map);
    memcpy(obj.slots, slots, slot_count * sizeof(cell));
    return obj;
  }

  auto to_string(const Object& obj, MapTable& maps, int level) -> std::string {
    std::string output;

    Map* obj_map = reinterpret_cast<Map*>(obj.map);
    output += " " + maps.get_string(obj_map->name) + " { ";

    for (size_t i = 0; i < obj_map->slot_count; ++i) {
      SlotDesc slot_desc = obj_map->slot_descs[i];
      output += " " + maps.get_string(slot_desc.name) + ": ";

      switch (slot_desc.type) {
        case SlotTypes::none:
          output += std::to_string(obj.slots[slot_desc.offset]) + ", ";
          break;
        case SlotTypes::parent: {
          cell parent_ptr = obj.slots[slot_desc.offset];
          Object* parent = reinterpret_cast<Object*>(parent_ptr);
          output += to_string(*parent, maps, level + 1) + ", ";
          break;
        }
        case SlotTypes::value:
          output += std::to_string(obj.slots[slot_desc.offset]) + ", ";
          break;
        default:
          output += "Unknown slot type, ";
      }
    }

    output += " }";
    return output;
  }

  namespace tests {
    auto all() -> bool {
      auto maps = MapTable();
      
      // Create Prototype Objects
      auto shape_name = maps.get_or_insert_string("shape");
      auto shape_x_name = maps.get_or_insert_string("x");
      auto shape_y_name = maps.get_or_insert_string("y");
      auto shape_x_accessor_name = maps.get_or_insert_string(":x");
      auto shape_y_accessor_name = maps.get_or_insert_string(":y");

      auto shape_slot_descs = std::vector<SlotDesc> {
        {shape_x_name, SlotTypes::value, 0},
        {shape_y_name, SlotTypes::value, 1},
        {shape_x_name, SlotTypes::accessor, 0},
        {shape_y_name, SlotTypes::accessor, 1},
      };

      auto shape_map = maps.create_map(shape_name, shape_slot_descs.data(), shape_slot_descs.size());
      auto shape_proto_obj = Object(shape_map);
      shape_proto_obj.set_property(shape_x_name, 0);
      shape_proto_obj.set_property(shape_y_name, 0);

      auto color_name = maps.get_or_insert_string("color");
      auto color_r_name = maps.get_or_insert_string("r");
      auto color_g_name = maps.get_or_insert_string("g");
      auto color_b_name = maps.get_or_insert_string("b");
      auto color_a_name = maps.get_or_insert_string("a");

      auto color_slot_descs = std::vector<SlotDesc> {
        {color_r_name, SlotTypes::value, 0},
        {color_g_name, SlotTypes::value, 1},
        {color_b_name, SlotTypes::value, 2},
        {color_a_name, SlotTypes::value, 3},
      };

      auto color_map = maps.create_map(color_name, color_slot_descs.data(), color_slot_descs.size());
      auto color_proto_obj = Object(color_map);
      color_proto_obj.set_property(color_r_name, 0);
      color_proto_obj.set_property(color_g_name, 0);
      color_proto_obj.set_property(color_b_name, 0);
      color_proto_obj.set_property(color_a_name, 255);

      auto cr_name = maps.get_or_insert_string("colored_rectangle");
      auto cr_width_name = maps.get_or_insert_string("width");
      auto cr_height_name = maps.get_or_insert_string("height");

      auto cr_slot_descs = std::vector<SlotDesc> {
        {cr_width_name, SlotTypes::value, 0},
        {cr_height_name, SlotTypes::value, 1},
        {shape_name, SlotTypes::parent, 2},
        {color_name, SlotTypes::parent, 3},
      };

      auto cr_map = maps.create_map(cr_name, cr_slot_descs.data(), cr_slot_descs.size());
      auto cr_proto_obj = Object(cr_map);
      cr_proto_obj.set_property(cr_width_name, 0);
      cr_proto_obj.set_property(cr_height_name, 0);
      cr_proto_obj.set_property(shape_name, reinterpret_cast<cell>(&shape_proto_obj));
      cr_proto_obj.set_property(color_name, reinterpret_cast<cell>(&color_proto_obj));

      // Create Instances
      auto shape_instance = shape_proto_obj.clone();
      shape_instance.set_property(shape_x_name, 420);
      shape_instance.set_property(shape_y_name, 69);

      auto color_instance = color_proto_obj.clone();
      auto cr_instance = cr_proto_obj.clone();
      cr_instance.set_property(shape_name, reinterpret_cast<cell>(&shape_instance));
      cr_instance.set_property(color_name, reinterpret_cast<cell>(&color_instance));
      cr_instance.set_property(cr_width_name, 666);
      cr_instance.set_property(cr_height_name, 333);

      color_instance.set_property(color_g_name, 100);
      color_instance.set_property(color_b_name, 200);

      std::cout << to_string(shape_proto_obj, maps) << std::endl;
      std::cout << to_string(color_proto_obj, maps) << std::endl;
      std::cout << to_string(cr_proto_obj, maps) << std::endl;
      std::cout << to_string(cr_instance, maps) << std::endl;

      return true;
    }


  } // namespace tests
  
}
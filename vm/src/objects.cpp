#include <kette/objects.hpp>
#include <kette/symbols.hpp>
#include <cassert>
#include <cstring>

namespace kette::oo {
  auto MapTable::get_next_map_id() -> cell {
    auto id = next_map_id;
    next_map_id++;
    return id;
  }

  auto MapTable::create_map(cell name, SlotDesc* descs, cell slots, cell value_slots) -> Map* {
    auto id = get_next_map_id();
    auto map = new Map;
    map->id = id;
    map->name = name;
    map->value_slot_count = value_slots;
    map->slot_count = slots;
    map->slot_descs = new SlotDesc[slots];
    memcpy(map->slot_descs, descs, slots * sizeof(SlotDesc));
    maps[id] = map;
    return map;
  }

  auto MapTable::get_map(cell id) -> Map* {
    return maps[id];
  }

  auto new_object(cell* map) -> Object {
    auto mapp = reinterpret_cast<Map*>(map);
    auto slots = new cell[mapp->value_slot_count];
    return Object{0, map, slots};
  }

  auto get_property(Object* obj, cell name) -> cell {
    auto mapp = reinterpret_cast<Map*>(obj->map);
    for (auto i = 0; i < mapp->slot_count; i++) {
      if (mapp->slot_descs[i].name == name)
        return obj->slots[mapp->slot_descs[i].offset];
    }
    for (auto i = 0; i < mapp->slot_count; i++) {
      if (mapp->slot_descs[i].type == SlotTypes::Parent) {
        auto parent = reinterpret_cast<Object*>(obj->slots[mapp->slot_descs[i].offset]);
        auto prop = get_property(parent, name);
        if (prop != -1) return prop;
      }
    }
    return -1;
  }

  auto set_property(Object* obj, cell name, cell value) -> cell {
    auto map = reinterpret_cast<Map*>(obj->map);
    for (auto i = 0; i < map->slot_count; i++) {
      if (map->slot_descs[i].name == name) {
        obj->slots[map->slot_descs[i].offset] = value;
        return 0;
      }
    }
    for (auto i = 0; i < map->slot_count; i++) {
      if (map->slot_descs[i].type == SlotTypes::Parent) {
        auto parent = reinterpret_cast<Object*>(obj->slots[map->slot_descs[i].offset]);
        auto prop = set_property(parent, name, value);
        if (prop != -1) return 0;
      }
    }
    return -1;
  }

  auto clone(Object* other) -> Object {
    // integer = 10, float = 01
    if ((other->mark & 0b11) != 0) {
      return Object { other->mark, other->map, other->slots};
    }
    auto mapp = reinterpret_cast<Map*>(other->map);
    
    auto slots = new cell[mapp->value_slot_count];
    memcpy(slots, other->slots, mapp->value_slot_count * sizeof(cell));
    return Object { other->mark, other->map, slots};
  }

  auto instantiate(Object* proto) -> Object {
    if ((proto->mark & 0b11) != 0) {
      return Object { proto->mark, proto->map, proto->slots};
    }
    auto map = reinterpret_cast<Map*>(proto->map);
    auto slots = new cell[map->value_slot_count];

    for (auto i = 0; i < map->slot_count; i++) {
      auto slot_desc = map->slot_descs[i];
      if (slot_desc.type != SlotTypes::Value && slot_desc.type != SlotTypes::Parent) continue;
      auto next = reinterpret_cast<Object*>(proto->slots[slot_desc.offset]);
      auto next_instance = instantiate(next);
      slots[slot_desc.offset] = reinterpret_cast<cell>(new Object { next_instance });
    }

    return Object { 0, proto->map, slots };
  }

  auto new_integer_object(i64 value, cell* map) -> Object {
    return Object { 0b10, map, reinterpret_cast<cell*>(value) };
  }

  auto new_float_object(f64 value, cell* map) -> Object {
    cell* lol;
    memcpy(&lol, &value, sizeof(lol));
    return Object { 0b01, map, lol };
  }


  auto to_string(const Object* obj, MapTable& maps, StringPool& strings) -> std::string {
    std::string output;

    auto map = reinterpret_cast<Map*>(obj->map);
    output += strings.get_string(map->name) + " {";
    
    if ((obj->mark & 0b10) != 0) {
      output += " " + std::to_string(reinterpret_cast<i64>(obj->slots));
    }
    if ((obj->mark & 0b01) != 0) {
      f64 lol;
      memcpy(&lol, &obj->slots, sizeof(f64));
      output += " " + std::to_string(lol);
    }
    for (auto i = 0; i < map->slot_count; i++) {
      auto slot_desc = map->slot_descs[i];
      if (slot_desc.type != SlotTypes::Value && slot_desc.type != SlotTypes::Parent) continue;
      output += " " + strings.get_string(slot_desc.name) + ": ";
      
      auto val = reinterpret_cast<Object*>(obj->slots[slot_desc.offset]);
      switch (slot_desc.type) {
        case SlotTypes::Value:
          output += to_string(val, maps, strings) + ", ";
          break;
        case SlotTypes::Parent:
          output += "(inherited) " + to_string(val, maps, strings);
          break;
      }
    }

    output += " }";
    return output;
  }


  // leaks memory because I don't care right now XD
  namespace tests {
    auto all() -> bool {
      auto strings = StringPool();
      auto maps = MapTable();
      auto integer_slots = std::vector<SlotDesc> {};
      auto integer_id = strings.string("integer");
      auto integer_map = maps.create_map(integer_id, integer_slots.data(), integer_slots.size(), 0);
      auto float_slots = std::vector<SlotDesc> {};
      auto float_id = strings.string("float");
      auto float_map = maps.create_map(float_id, float_slots.data(), float_slots.size(), 0);
      
      auto int_1234 = new_integer_object(1234, reinterpret_cast<cell*>(integer_map));
      auto float_1234_69 = new_float_object(1234.69, reinterpret_cast<cell*>(float_map));
      std::cout << "Test Integer Object:\n  " << to_string(&int_1234, maps, strings) << std::endl;
      std::cout << "Test Float Object:\n  " << to_string(&float_1234_69, maps, strings) << std::endl;

      auto int_clone = clone(&int_1234);
      std::cout << "Test Cloning Object:\n  " << to_string(&int_clone, maps, strings) << std::endl;

      auto data_slots = std::vector<SlotDesc> {
        {strings.string("users"), SlotTypes::Value, 0},
        {strings.string("programmers"), SlotTypes::Value, 1},
      };
      auto data_map = maps.create_map(strings.string("datastruct"), data_slots.data(), data_slots.size(), 2);
      auto data_proto = new_object(reinterpret_cast<cell*>(data_map));
      auto data_proto_users = new Object(new_integer_object(0, reinterpret_cast<cell*>(integer_map)));
      auto data_proto_programmers = new Object(new_integer_object(420, reinterpret_cast<cell*>(integer_map)));
      set_property(&data_proto, strings.string("users"), reinterpret_cast<cell>(data_proto_users));
      set_property(&data_proto, strings.string("programmers"), reinterpret_cast<cell>(data_proto_programmers));
      std::cout << "Test creating (prototype) object with values:\n  " << to_string(&data_proto, maps, strings) << std::endl;

      auto data_instance = instantiate(&data_proto);
      std::cout << "Test instantiate object from (prototype) object:\n  " << to_string(&data_instance, maps, strings) << std::endl;
      auto data_instance_users = reinterpret_cast<Object*>(get_property(&data_instance, strings.string("users")));
      data_instance_users->slots = reinterpret_cast<cell*>(69);
      std::cout << "Test change value from instantiated object:\n  " << to_string(&data_instance, maps, strings) << std::endl;
      std::cout << "Test original prototype value unchanged:\n  " << to_string(&data_proto, maps, strings) << std::endl;
    //   // Create Prototype Objects
    //   auto shape_name = maps.get_or_insert_string("shape");
    //   auto shape_x_name = maps.get_or_insert_string("x");
    //   auto shape_y_name = maps.get_or_insert_string("y");
    //   auto shape_x_accessor_name = maps.get_or_insert_string(":x");
    //   auto shape_y_accessor_name = maps.get_or_insert_string(":y");

    //   auto shape_slot_descs = std::vector<SlotDesc> {
    //     {shape_x_name, SlotTypes::value, 0},
    //     {shape_y_name, SlotTypes::value, 1},
    //     {shape_x_name, SlotTypes::accessor, 0},
    //     {shape_y_name, SlotTypes::accessor, 1},
    //   };

    //   auto shape_map = maps.create_map(shape_name, shape_slot_descs.data(), shape_slot_descs.size(), 0);
    //   auto shape_proto_obj = Object(shape_map);
    //   shape_proto_obj.set_property(shape_x_name, 0);
    //   shape_proto_obj.set_property(shape_y_name, 0);

    //   auto color_name = maps.get_or_insert_string("color");
    //   auto color_r_name = maps.get_or_insert_string("r");
    //   auto color_g_name = maps.get_or_insert_string("g");
    //   auto color_b_name = maps.get_or_insert_string("b");
    //   auto color_a_name = maps.get_or_insert_string("a");

    //   auto color_slot_descs = std::vector<SlotDesc> {
    //     {color_r_name, SlotTypes::value, 0},
    //     {color_g_name, SlotTypes::value, 1},
    //     {color_b_name, SlotTypes::value, 2},
    //     {color_a_name, SlotTypes::value, 3},
    //   };

    //   auto color_map = maps.create_map(color_name, color_slot_descs.data(), color_slot_descs.size(), 0);
    //   auto color_proto_obj = Object(color_map);
    //   color_proto_obj.set_property(color_r_name, 0);
    //   color_proto_obj.set_property(color_g_name, 0);
    //   color_proto_obj.set_property(color_b_name, 0);
    //   color_proto_obj.set_property(color_a_name, 255);

    //   auto cr_name = maps.get_or_insert_string("colored_rectangle");
    //   auto cr_width_name = maps.get_or_insert_string("width");
    //   auto cr_height_name = maps.get_or_insert_string("height");

    //   auto cr_slot_descs = std::vector<SlotDesc> {
    //     {cr_width_name, SlotTypes::value, 0},
    //     {cr_height_name, SlotTypes::value, 1},
    //     {shape_name, SlotTypes::parent, 2},
    //     {color_name, SlotTypes::parent, 3},
    //   };

    //   auto cr_map = maps.create_map(cr_name, cr_slot_descs.data(), cr_slot_descs.size(), 0);
    //   auto cr_proto_obj = Object(cr_map);
    //   cr_proto_obj.set_property(cr_width_name, 0);
    //   cr_proto_obj.set_property(cr_height_name, 0);
    //   cr_proto_obj.set_property(shape_name, reinterpret_cast<cell>(&shape_proto_obj));
    //   cr_proto_obj.set_property(color_name, reinterpret_cast<cell>(&color_proto_obj));

    //   // Create Instances
    //   auto shape_instance = shape_proto_obj.clone();
    //   shape_instance.set_property(shape_x_name, 420);
    //   shape_instance.set_property(shape_y_name, 69);

    //   auto color_instance = color_proto_obj.clone();
    //   auto cr_instance = cr_proto_obj.clone();
    //   cr_instance.set_property(shape_name, reinterpret_cast<cell>(&shape_instance));
    //   cr_instance.set_property(color_name, reinterpret_cast<cell>(&color_instance));
    //   cr_instance.set_property(cr_width_name, 666);
    //   cr_instance.set_property(cr_height_name, 333);

    //   color_instance.set_property(color_g_name, 100);
    //   color_instance.set_property(color_b_name, 200);

    //   std::cout << to_string(shape_proto_obj, maps) << std::endl;
    //   std::cout << to_string(color_proto_obj, maps) << std::endl;
    //   std::cout << to_string(cr_proto_obj, maps) << std::endl;
    //   std::cout << to_string(cr_instance, maps) << std::endl;

    //   auto test = new Object(cr_map);
    
    //   auto tagged = add_tag(test, TagValues::Integer);
    //   test->set_property(cr_width_name, 10);
    //   auto detagged = remove_tag(tagged);
    //   std::cout << detagged->get_property(cr_width_name) << std::endl;

      return true;
    }

  } // namespace tests
  
}
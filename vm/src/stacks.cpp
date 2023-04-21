#include <kette/stacks.h>
#include <kette/mem.hpp>
#include <cstring>

namespace kette {
  Stack::Stack(usize size) {
    auto [alloc, allocSize] = mem::allocate_memory(size, false);
    allocation = alloc;
    capacity = allocSize;
    length = 0;
  }

  Stack::~Stack() {
    mem::free_memory(allocation, capacity);
  }

  auto Stack::push(cell ptr) -> usize {
    if (length == capacity) {
      auto [alloc, allocSize] = mem::allocate_memory(capacity * 2, false);
      std::memcpy(alloc, allocation, capacity);
      mem::free_memory(allocation, capacity);
      allocation = alloc;
      allocSize = capacity;
    }
    auto offset = reinterpret_cast<cell*>(allocation) + length;
    *offset = ptr;
    auto index = length;
    length++;
    return index;
  }

  auto Stack::pop() -> cell {
    auto offset = reinterpret_cast<cell*>(allocation) + length;
    length--;
    auto ptr = *offset;
    return ptr;
  }

  auto Stack::index(usize index) -> cell {
    auto offset = reinterpret_cast<cell*>(allocation) + index;
    auto ptr = *offset;
    return ptr;
  }
}
#include <kette/defaults.hpp>
#include <kette/mem.hpp>
#include <cstring>
#include <sstream>
#include <iomanip>

#ifdef __unix__
#include <stdlib.h>
#include <sys/mman.h>
#include <unistd.h>
#elif defined(_WIN64)
#include <windows.h>
#else
#error "Unsupported platform"
#endif

namespace kette::mem {
  ExecutableMemory::ExecutableMemory(usize size) {
    length   = 0;
    auto [newAllocation, newCapacity] = allocate_memory(size, true);
    allocation = newAllocation;
    capacity = newCapacity;
  }

  ExecutableMemory::~ExecutableMemory() {
    free_memory(allocation, capacity);
  }

  auto ExecutableMemory::resize(usize size) -> void {
    auto [newAllocation, newCapacity] = allocate_memory(size, true);
    std::memcpy(newAllocation, allocation, capacity);
    free_memory(allocation, capacity);
    allocation = newAllocation;
    capacity = newCapacity;
  }

  auto ExecutableMemory::push(u8 value) -> void {
    if (length == capacity) {
      resize(capacity * 2);
    }
    auto ptr = reinterpret_cast<u8*>(allocation) + length;
    *ptr = value;
    length++;
  }

  auto ExecutableMemory::push_span(std::span<u8> span) -> void {
    if (length + span.size() >= capacity) {
      resize(capacity * 2);
    }
    auto ptr = reinterpret_cast<u8*>(allocation) + length;
    std::memcpy(ptr, span.data(), span.size());
    length += span.size();
  }

  auto ExecutableMemory::span() -> std::span<u8> {
    return std::span<u8> {static_cast<u8*>(allocation), length};
  }

  auto ExecutableMemory::to_string() -> std::string {
    auto data = span();
    std::ostringstream oss;
    oss << "Executable Memory {" << capacity << ", " << length << ", [";
    oss << std::hex << std::setfill('0');
    for (auto c : data) {
      oss << "0x" << std::setw(2) << static_cast<int>(c) << ", ";
    }
    oss << "]}";
    return oss.str();
  }

  auto ExecutableMemory::to_function() -> int(*)() {
    return reinterpret_cast<int(*)()>(allocation);
  }

  auto allocate_memory(usize preceiled_size, bool exec) -> std::tuple<void*, usize> {
    void* allocation;
    auto size = ceil(preceiled_size, PAGE_SIZE);
#ifdef __unix__
    posix_memalign(&allocation, PAGE_SIZE, size);
    auto prot = PROT_EXEC | PROT_READ;
    mprotect(allocation, size, exec ? PROT_EXEC | PROT_READ | PROT_WRITE : PROT_READ | PROT_WRITE);
#elif defined(_WIN64)
    allocation = VirtualAlloc(nullptr, size, MEM_COMMIT | MEM_RESERVE, exec ? PAGE_EXECUTE_READWRITE : PAGE_READWRITE);
#else
#error "Unsupported platform"
#endif
    return {allocation, size};
  }

  auto free_memory(void* allocation, usize size) -> void {
#ifdef __unix__
    free(allocation);
#elif defined(_WIN64)
    VirtualFree(allocation, size, MEM_RELEASE);
#else
#error "Unsupported platform"
#endif
  }
} // namespace kette::mem
#include <kette/defaults.hpp>
#include <kette/mem.hpp>
#include <cstring>
#include <iostream>
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
    capacity = ceil(size, PAGE_SIZE);
#ifdef __unix__
    posix_memalign(&allocation, PAGE_SIZE, capacity);
    mprotect(allocation, capacity, PROT_EXEC | PROT_READ | PROT_WRITE);
#elif defined(_WIN64)
    allocation = VirtualAlloc(nullptr, capacity, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
#error "Unsupported platform"
#endif
  }
  ExecutableMemory::~ExecutableMemory() {
#ifdef __unix__
    free(allocation);
#elif defined(_WIN64)
    VirtualFree(allocation, capacity, MEM_RELEASE);
#else
#error "Unsupported platform"
#endif
  }

  auto ExecutableMemory::resize(usize size) -> void {
    auto new_capacity = ceil(size, PAGE_SIZE);
    void* new_allocation;
#ifdef __unix__
    posix_memalign(&new_allocation, PAGE_SIZE, new_capacity);
    mprotect(new_allocation, new_capacity, PROT_EXEC | PROT_READ | PROT_WRITE);
#elif defined(_WIN64)
    new_allocation = VirtualAlloc(nullptr, new_capacity, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
#else
#error "Unsupported platform"
#endif
    std::memcpy(new_allocation, allocation, capacity);
#ifdef __unix__
    free(allocation);
#elif defined(_WIN64)
    VirtualFree(allocation, capacity, MEM_RELEASE);
#else
#error "Unsupported platform"
#endif
    allocation = new_allocation;
    capacity = new_capacity;
  }
} // namespace kette::mem
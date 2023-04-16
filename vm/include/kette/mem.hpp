#pragma once

#include <kette/defaults.hpp>

#ifdef __unix__
#include <unistd.h>
#include <stdlib.h>
#elif defined(_WIN64)
#include <windows.h>
#else
#error "Unsupported platform"
#endif

namespace kette::mem {
#ifdef __unix__
  const auto PAGE_SIZE = static_cast<usize>(sysconf(_SC_PAGE_SIZE));
#elif defined(_WIN64)
  const auto PAGE_SIZE = []() {
    auto sys_info = SYSTEM_INFO{};
    GetSystemInfo(&sys_info);
    return static_cast<usize>(sys_info.dwPageSize);
  }();
#else
#error "Unsupported platform"
#endif

  struct ExecutableMemory {
  private:
    void *allocation;
    usize capacity;
    usize length;

  public:
    ExecutableMemory(usize size);
    ~ExecutableMemory();
    auto resize(usize size) -> void;
  };

  template <typename T> 
  auto ceil(T size, T ceiling) -> T {
    return (size / ceiling + 1) * ceiling;
  }

} // namespace kette::mem
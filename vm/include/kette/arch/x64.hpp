#pragma once
#include <kette/defaults.hpp>

namespace kette::arch {

  // int regs = 0-8, 0
  // ex int regs = 0-8, 1
  struct Register {
    usize id, groupId; 
  };



  using Operator = std::variant<>;
}
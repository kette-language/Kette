#pragma once
#include <kette/defaults.hpp>

namespace kette::bytecode {
  enum BytecodeKind {
    Invalid,
    Push,
    Pop,
    Send,
    BuiltinOp,
    Branch,
    BranchTrue,
    BranchFalse,
    Return,
  };

  struct Bytecode {
    BytecodeKind kind;
    cell value1, value2;
  };
}

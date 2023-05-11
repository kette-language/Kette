#pragma once

#include <kette/defaults.hpp>
#include <kette/symbols.hpp>
#include <kette/cfgs.hpp>
#include <kette/reader.hpp>
#include <kette/stacks.hpp>
#include <kette/objects.hpp>

namespace kette {
  class Context {
  private:
    SymbolTable* symbols;
    oo::MapTable* maps;
    CFGS* cfgs;
    Stack* datastack;
    Stack* retainstack;
    Stack* callstack;
  public:
    Context();
    ~Context();

    auto execute(std::string_view str) -> void;
    auto addBuiltinSymbols() -> void;
  };
}
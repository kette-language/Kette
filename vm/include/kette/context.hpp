#pragma once

#include <kette/defaults.hpp>
#include <kette/symbols.hpp>
#include <kette/cfgs.hpp>
#include <kette/reader.hpp>

namespace kette {
  class Context {
  private:
    SymbolTable* symbols;
    CFGS* cfgs;
  public:
    Context();
    ~Context();

    auto execute(std::string_view str) -> void;
    auto addBuiltinSymbols() -> void;
  };
}
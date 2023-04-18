#include <kette/defaults.hpp>
#include <kette/symbols.hpp>

namespace kette {
  class Context {
  private:
    SymbolTable* symbols;
  
  public:
    Context();
    ~Context();

    auto execute(std::string_view str) -> void;
    auto addBuiltinSymbols() -> void;
  };
}
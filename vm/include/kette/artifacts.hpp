#pragma once
#include <kette/defaults.hpp>
#include <kette/mem.hpp>
#include <kette/symbols.hpp>

namespace kette {
  using ArtifactId = usize;
  
  struct Artifact {
    mem::ExecutableMemory memory;
    std::map<SymbolId, usize> offsets;
  };
  
  class Artifacts {
  private:
    std::map<SymbolId, ArtifactId> mappings;
    std::map<ArtifactId, Artifact> artifacts;  
  };
}
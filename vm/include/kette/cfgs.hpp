#pragma once

#include <kette/defaults.hpp>
#include <kette/symbols.hpp>
#include <kette/reader.hpp>

namespace kette {
  using CFGTreeId = usize;
  using CFGNodeId = usize;

  struct CFGNumberNode {
    NumberWord value;
  };

  struct CFGStringNode {
    std::string value;
  };

  struct CFGUnknownSymbolNode {
    std::string value;
  };

  struct CFGSymbolNode {
    SymbolId id;
  };

  struct CFGQuotationNode {
    std::vector<CFGNodeId> body;
    bool intheline, recursive; // inline is a keyword lol
  };

  struct CFGTreeRefNode {
    CFGTreeId treeId;
    CFGNodeId nodeId;
  };

  struct CFGRootNode {
    std::vector<CFGNodeId> nodes;
  };

  using CFGNode = std::variant<CFGRootNode, CFGTreeRefNode, CFGQuotationNode, CFGUnknownSymbolNode, 
              CFGSymbolNode, CFGStringNode, CFGNumberNode>;

  class CFGTree {
  private:
    std::map<SymbolId, CFGNodeId> mappings;
    std::map<CFGNodeId, CFGNode*> nodes;
    CFGNodeId nextId;
  
  private:
    auto getNextId() -> CFGNodeId;
  
  public:
    auto makeRootNode() -> std::tuple<CFGNodeId, CFGNode*>;
    auto insertNode(CFGNode* node) -> CFGNodeId;
    auto toString() -> std::string;
  };


  class CFGS {
  private:
    std::map<SymbolId, CFGTreeId> mappings;
    std::map<CFGTreeId, std::shared_ptr<CFGTree>> trees;
    CFGTreeId nextId;
  
  private:
    auto getNextId() -> CFGNodeId;
  
  public:
    auto newTree() -> std::tuple<CFGTreeId, std::shared_ptr<CFGTree>>;
  };
} // namespace kette
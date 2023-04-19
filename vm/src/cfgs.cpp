#include <kette/cfgs.hpp>

namespace kette {

  auto CFGTree::getNextId() -> CFGNodeId {
    auto id = nextId;
    nextId++;
    return id;
  }

  auto CFGTree::makeRootNode() -> std::tuple<CFGNodeId, CFGNode*> {
    auto id = getNextId();
    auto node = new CFGNode { CFGRootNode {} } ;
    nodes[id] = node;
    return { id, node };
  }

  auto CFGTree::insertNode(CFGNode* node) -> CFGNodeId {
    auto id = getNextId();
    nodes[id] = node;
    return id;
  }

  auto CFGTree::toString() -> std::string {
    return std::string {};
  }
  
  auto CFGS::getNextId() -> CFGTreeId {
    auto id = nextId;
    nextId++;
    return id;
  }

  auto CFGS::newTree() -> std::tuple<CFGTreeId, std::shared_ptr<CFGTree>> {
    auto id = getNextId();
    auto tree = std::make_shared<CFGTree>();
    trees[id] = tree;
    return {id, tree};
  }
}
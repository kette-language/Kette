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

  auto to_string_helper(const CFGNode& node, const std::map<CFGNodeId, CFGNode*>& nodes) -> std::string {
    std::string result;
    if (const auto* root_node = std::get_if<CFGRootNode>(&node)) {
      for (const auto& child_id : root_node->nodes) {
        result += to_string_helper(*nodes.at(child_id), nodes);
        result += "\n";
      }
    } else if (const auto* quotation_node = std::get_if<CFGQuotationNode>(&node)) {
      result += "QuotationNode: ";
      for (const auto& child_id : quotation_node->body) {
        result += to_string_helper(*nodes.at(child_id), nodes);
        result += "\n";
      }
      result += "inline: " + std::to_string(quotation_node->intheline) + ", ";
      result += "recursive: " + std::to_string(quotation_node->recursive);
    } else if (const auto* symbol_node = std::get_if<CFGSymbolNode>(&node)) {
      result += "SymbolNode: ";
      result += std::to_string(symbol_node->id);
    } else if (const auto* string_node = std::get_if<CFGStringNode>(&node)) {
      result += "StringNode: ";
      result += string_node->value;
    } else if (const auto* unknown_node = std::get_if<CFGUnknownSymbolNode>(&node)) {
      result += "UnkownSymbolNode: ";
      result += unknown_node->value;
    } else if (const auto* number_node = std::get_if<CFGNumberNode>(&node)) {
      result += "NumberNode: ";
      if (const auto* number = std::get_if<u64>(&number_node->value)) {
        result += std::to_string(*number);
        result += " [unsigned integer]";
      } else if (const auto* number = std::get_if<i64>(&number_node->value)) {
        result += std::to_string(*number);
        result += " [signed integer]";
      } else if (const auto* number = std::get_if<f64>(&number_node->value)) {
        result += std::to_string(*number);
        result += " [floating number]";
      }
    }
    return result;
  }

  auto CFGTree::to_string() -> std::string {
    std::string result;
    for (const auto& entry : nodes) {
      const auto& node = *entry.second;
      result += to_string_helper(node, nodes);
      result += "\n";
    }
    return result;
  }
}
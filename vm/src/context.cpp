#include <kette/context.hpp>

namespace kette {
  Context::Context() {
    symbols = new SymbolTable;
    maps = new oo::MapTable();
    cfgs = new CFGS;
    datastack = new Stack(4096);
    retainstack = new Stack(4096);
    callstack = new Stack(4096);
  }

  Context::~Context() {
    delete symbols;
    delete cfgs;
  }

  auto Context::execute(std::string_view str) -> void {
    auto reader = Reader(str);
    auto [treeId, treePtr] = cfgs->newTree();
    auto tree = treePtr;
    auto [rootId, root] = treePtr->makeRootNode();

    for (auto word = reader.readWord(); !word.isNull(); word = reader.readWord()) {
      std::visit(match {
        [&](IdentifierWord const& val) { 
          if (val.value == "macro:") {
            printf("todo: define reader macro\n");
            return;
          }
          if (symbols->contains(val.value)) {
            auto [symId, sym] = symbols->get(val.value);
            if (sym->kind == SymbolKind::ReaderMacro) {
              printf("todo: execute reader macro");
            } else if (sym->kind == SymbolKind::Builtin) {
              auto node = new CFGNode { CFGSymbolNode { symId } };
              tree->insertNode(node);
            } else {
              printf("todo: macro instance\n");
              return;
            }
          } else {
            auto node = new CFGNode { CFGUnknownSymbolNode { val.value } };
            tree->insertNode(node);
          }
        },
        [tree](StringWord const& val) { 
          auto node = new CFGNode { CFGStringNode { val.value } };
          tree->insertNode(node);
        },
        [tree](NumberWord const& val) { 
          auto node = new CFGNode { CFGNumberNode { val } };
          tree->insertNode(node);
        },
        [](auto) { },
      }, word.data);
    }

    std::cout << tree->to_string() << std::endl;
  }

  auto Context::addBuiltins() -> void {
    symbols->insert(Symbol { "+", SymbolKind::Builtin });
    symbols->insert(Symbol { "-", SymbolKind::Builtin });
    symbols->insert(Symbol { "*", SymbolKind::Builtin });
    symbols->insert(Symbol { "/", SymbolKind::Builtin });
    symbols->insert(Symbol { ".", SymbolKind::Builtin });
    symbols->insert(Symbol { "drop", SymbolKind::Builtin });
    symbols->insert(Symbol { "dup", SymbolKind::Builtin });
    symbols->insert(Symbol { "2dup", SymbolKind::Builtin });
    symbols->insert(Symbol { "swap", SymbolKind::Builtin });
    symbols->insert(Symbol { "2swap", SymbolKind::Builtin });
    symbols->insert(Symbol { "call", SymbolKind::Builtin});
    symbols->insert(Symbol { "over", SymbolKind::Builtin });
    symbols->insert(Symbol { "2over", SymbolKind::Builtin });
    symbols->insert(Symbol { "rot", SymbolKind::Builtin });
    symbols->insert(Symbol { "2rot", SymbolKind::Builtin });
    symbols->insert(Symbol { "pick", SymbolKind::Builtin });
    symbols->insert(Symbol { "2pick", SymbolKind::Builtin });
    symbols->insert(Symbol { "dip", SymbolKind::Builtin });
    
    auto id_colon = symbols->insert(Symbol { ":", SymbolKind::ReaderMacro });
    auto id_string = symbols->insert(Symbol { "\"", SymbolKind::ReaderMacro });
    auto id_backslach = symbols->insert(Symbol { "\\", SymbolKind::ReaderMacro});
    // todo: write reader macros in C++

    auto entry_name = maps->string("entry");
    auto source_name = maps->string("source");
    auto name_name = maps->string("name");
    auto stack_effect_name = maps->string("stack-effect");
    
    auto executable_name = maps->string("executable");
    auto executable_trait_slot_desc = new std::vector<oo::SlotDesc> {
      { entry_name, oo::SlotTypes::value, 0},
    };
    auto executable_map = maps->create_map(executable_name, executable_trait_slot_desc->data(), executable_trait_slot_desc->size(), 0);
    auto executable_object = new oo::Object(executable_map);
    executable_object->set_property(entry_name, 0);
    prototypes[executable_name] = executable_object;
    
    auto quotation_name = maps->string("builtin-quotation");
    auto quotation_slots_desc = new std::vector<oo::SlotDesc> {
      { executable_name, oo::SlotTypes::parent, 0},
    };
    auto quotation_map = maps->create_map(quotation_name, quotation_slots_desc->data(), quotation_slots_desc->size(), 0);
    auto quotation_object = new oo::Object(quotation_map);
    quotation_object->set_property(executable_name, reinterpret_cast<cell>(executable_object));
    prototypes[executable_name] = executable_object;

    auto word_name = maps->string("builtin-word");
    auto word_slots_desc = new std::vector<oo::SlotDesc> {
      { name_name, oo::SlotTypes::value, 0},
      { stack_effect_name, oo::SlotTypes::value, 1},
      { executable_name, oo::SlotTypes::parent, 2},
    };
    auto word_map = maps->create_map(word_name, word_slots_desc->data(), word_slots_desc->size(), 0);
    auto word_object = new oo::Object(quotation_map);
    quotation_object->set_property(executable_name, 0);
    quotation_object->set_property(stack_effect_name, 0);
    quotation_object->set_property(executable_name, reinterpret_cast<cell>(executable_object));
    prototypes[word_name] = executable_object;

  }
} // namespace kette
#include <kette/context.hpp>
#include <kette/builtins.hpp>
namespace kette {
  Context::Context() {
    symbols = new SymbolTable;
    maps = new oo::MapTable();
    cfgs = new CFGS;
    datastack = new Stack(4096);
    retainstack = new Stack(4096);
    callstack = new Stack(4096);
    // prototypes = new Stack(4096);
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
    symbols->insert({ "+", SymbolKind::Builtin });
    symbols->insert({ "-", SymbolKind::Builtin });
    symbols->insert({ "*", SymbolKind::Builtin });
    symbols->insert({ "/", SymbolKind::Builtin });
    symbols->insert({ ".", SymbolKind::Builtin });
    symbols->insert({ "drop", SymbolKind::Builtin });
    symbols->insert({ "dup", SymbolKind::Builtin });
    symbols->insert({ "2dup", SymbolKind::Builtin });
    symbols->insert({ "swap", SymbolKind::Builtin });
    symbols->insert({ "2swap", SymbolKind::Builtin });
    symbols->insert({ "call", SymbolKind::Builtin});
    symbols->insert({ "over", SymbolKind::Builtin });
    symbols->insert({ "2over", SymbolKind::Builtin });
    symbols->insert({ "rot", SymbolKind::Builtin });
    symbols->insert({ "2rot", SymbolKind::Builtin });
    symbols->insert({ "pick", SymbolKind::Builtin });
    symbols->insert({ "2pick", SymbolKind::Builtin });
    symbols->insert({ "dip", SymbolKind::Builtin });
    
    symbols->insert({ "datastack>>", SymbolKind::Builtin });
    symbols->insert({ "retainstack>>", SymbolKind::Builtin });
    symbols->insert({ "callstack>>", SymbolKind::Builtin });

    symbols->insert({ ">r", SymbolKind::Builtin });
    symbols->insert({ "r>", SymbolKind::Builtin });

    symbols->insert({ ">c", SymbolKind::Builtin });
    symbols->insert({ "c>", SymbolKind::Builtin });

    symbols->insert({ "malloc", SymbolKind::Builtin });
    symbols->insert({ "free", SymbolKind::Builtin });

    symbols->insert({ "@", SymbolKind::Builtin });
    symbols->insert({ "!", SymbolKind::Builtin });

    symbols->insert({ "@n", SymbolKind::Builtin });
    symbols->insert({ "!n", SymbolKind::Builtin });

    symbols->insert({ "syscall0", SymbolKind::Builtin });
    symbols->insert({ "syscall1", SymbolKind::Builtin });
    symbols->insert({ "syscall2", SymbolKind::Builtin });
    symbols->insert({ "syscall3", SymbolKind::Builtin });
    symbols->insert({ "syscall4", SymbolKind::Builtin });
    symbols->insert({ "syscall5", SymbolKind::Builtin });
    symbols->insert({ "syscall6", SymbolKind::Builtin });

    auto id_colon = symbols->insert(Symbol { ":", SymbolKind::ReaderMacro });
    auto id_string = symbols->insert(Symbol { "\"", SymbolKind::ReaderMacro });
    auto id_backslach = symbols->insert(Symbol { "\\", SymbolKind::ReaderMacro});
    // // todo: write reader macros in C++

    // auto executable_trait_slot_desc = new std::vector<oo::SlotDesc> {
    //   { strings.string("entry"), oo::SlotTypes::value, 0},
    // };
    // auto executable_map = maps->create_map(
    //   strings.string("executable"), 
    //   executable_trait_slot_desc->data(), 
    //   executable_trait_slot_desc->size(), 0);
    // auto executable_object = new oo::Object(executable_map);
    // executable_object->set_property(strings.string("entry"), 0);
    // prototypes[strings.string("executable")] = executable_object;
    
    // auto quotation_slots_desc = new std::vector<oo::SlotDesc> {
    //   { strings.string("executable"), oo::SlotTypes::value, 0},
    // };
    // auto quotation_map = maps->create_map(
    //   strings.string("quotation"), 
    //   quotation_slots_desc->data(), 
    //   quotation_slots_desc->size(), 0);
    // auto quotation_object = new oo::Object(quotation_map);
    // quotation_object->set_property(strings.string("executable"), reinterpret_cast<cell>(executable_object));
    // prototypes[strings.string("quotation")] = quotation_object;

    // auto word_slots_desc = new std::vector<oo::SlotDesc> {
    //   { strings.string("name"), oo::SlotTypes::value, 0},
    //   { strings.string("stack-effect"), oo::SlotTypes::value, 1},
    //   { strings.string("executable"), oo::SlotTypes::value, 2},
    //   { strings.string("annotations"), oo::SlotTypes::value, 3 },
    // };
    // auto word_map = maps->create_map(
    //   strings.string("word"), 
    //   word_slots_desc->data(), 
    //   word_slots_desc->size(), 0);
    // auto word_object = new oo::Object(quotation_map);
    // word_object->set_property(strings.string("name"), 0);
    // word_object->set_property(strings.string("stack-effect"), 0);
    // word_object->set_property(strings.string("executable"), reinterpret_cast<cell>(executable_object));
    // prototypes[strings.string("word")] = word_object;


    // auto dot_word_executable_object = new oo::Object(word_object->clone());
    // dot_word_executable_object->set_property(entry_name, reinterpret_cast<cell>(builtins::print_num));
    // auto dot_word_object = new oo::Object(word_object->clone());
    // word_object->set_property(strings.string("name"), strings.string("."));
    // word_object->set_property(executable_name, reinterpret_cast<cell>(dot_word_executable_object));


    // auto fixnum_trait_name = maps->string("fixnum-traits");
    // auto fixnum_trait_slot_desc = new std::vector<oo::SlotDesc> {
    //   { fixnum_trait_name, oo::SlotTypes::function, 0}
    // };
    // auto fixnum_trait_map = maps->create_map(fixnum_trait_name, fixnum_trait_slot_desc->data(), fixnum_trait_slot_desc->size(), 0);
    // auto fixnum_trait_object = new oo::Object(fixnum_trait_map);
    // prototypes[fixnum_trait_name] = fixnum_trait_object;

    // auto fixnum_name = maps->string("fixnum");
    // auto fixnum_trait_slot_desc = new std::vector<oo::SlotDesc> {
    //   { fixnum_trait_name, oo::SlotTypes::parent, 0},
    //   { value_name, oo::SlotTypes::value, 1 },
    //   { value_accessor, oo::SlotTypes::accessor, 2},
    // };
    // auto fixnum_map = maps->create_map(fixnum_name, fixnum_trait_slot_desc->data(), fixnum_trait_slot_desc->size(), 0);
    // auto fixnum_object = new oo::Object(fixnum_map);
    // fixnum_object->set_property(entry_name, 0);
    // prototypes[fixnum_name] = fixnum_object;


  }

} // namespace kette
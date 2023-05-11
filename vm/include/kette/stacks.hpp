#include <kette/defaults.hpp>

namespace kette {
  class Stack {
  private:
    void* allocation;
    usize capacity;
    usize length;
  
  public:
    Stack(usize size);
    ~Stack();

    auto push(cell ptr) -> usize;
    auto pop() -> cell;
    auto index(usize index) -> cell; 
  };
}
#pragma once
#include <stdint.h>
#include <variant>
#include <tuple>
#include <span>
#include <string>
#include <optional>
#include <array>
#include <vector>
#include <memory>
#include <map>

#include <iostream>
#include <cstdio>

using i8  = char;
using i16 = short;
using i32 = int;
using i64 = long long;

using u8  = unsigned char;
using u16 = unsigned short;
using u32 = unsigned int;
using u64 = unsigned long long;

using f32 = float;
using f64 = double;

using usize = uintptr_t;
using isize = intptr_t;

using cell = uintptr_t;

template<class... Ts> struct match : Ts... { using Ts::operator()...; };
template<class... Ts> match(Ts...) -> match<Ts...>;
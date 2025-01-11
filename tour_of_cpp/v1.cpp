#include <iostream>

constexpr int square(int x) {
    return x * x;
}
// Can be applied to functions, making them capable of being evaluated at compile time if used with constant expressions.
// Such functions must have a single return statement and no side effects
constexpr int result = square(5); // COMPILE TIME evaluation
// For modern C++ (C++11 and later), prefer constexpr over const when possible, as it 
// provides stronger guarantees and can help optimize your code.

int main() {
    int b = 1.1; // no error // b=1
    // int a {1.1}; // error
    int x {true};
    auto y = true;
    std::cout << y << std::endl;

    const int _x = 10; // Known at compile time
    const int _y = std::rand(); // Known only at runtime
    // constexpr int y = std::rand(); // Error: std::rand() cannot be evaluated at compile time

    // RUNTIME evaluation
    int runtimeValue = 10; 
    int runtimeResult = square(runtimeValue); // Computed at runtime

    int v[] = {0, 1, 2, 3};
    for(auto i: v){
        std::cout << i << std::endl;
    }
    for(auto &i: v){ 
    // A reference issimilar to a pointer, except that you don’t need to use a prefix * to accessthe value referred to by the reference
        ++i;
    } printf("");
    for(auto &i: v){
        std::cout << i << std::endl;
    }

    double* p = nullptr;
    // int x = nullptr; // Error: int is not a pointer
    // In older code, o or NULL is typically used instead of nullptr. However, us-
    // ing nullptr eliminates potential confusion between integers (such as o or
    // NULL) and pointers (such as nullptr).
    // There is no “null reference.” A reference must refer to a valid object 
    // (and implementations assume that it does). There are obscure and clever ways to 
    // violate that rule; don’t do that.

    // `new` keyword and `delete` keyword

    // We use . (dot) to access struct members through a name (and through a
    // reference) and -> to access struct members through a pointer. For
    // example:

    return 0;
}
// consteval: STRICTLY COMPILE TIME UNLIKE constexpr
// consteval: C++ 20
// consteval int add(int a, int b) {
//     return a + b;
// }

// constexpr int compute(int x) {
//     return add(x, 5); // This works because `add` is evaluated at compile time
// }

// int main() {
//     constexpr int result = add(3, 4); // OK: Compile-time evaluation
//     int runtime_value = 10;
//     // int invalid = add(runtime_value, 4); // Error: Cannot call a consteval function with runtime arguments
// }

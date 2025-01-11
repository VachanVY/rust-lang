#include <iostream>

class Vector {
    double* elem;
    int sz;
    public:
        Vector(int s) 
            : elem {new double[s]}, sz {s} {} 
};

enum {
    hi, yo, hello
}; // just hi //yo // hello

enum class Color {
    red, green, blue 
}; // Color::red // Color::green // Color::blue

int main(){
    int x = int(Color::red);
    throw 
}
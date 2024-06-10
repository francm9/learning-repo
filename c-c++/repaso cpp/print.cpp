#include <iostream>
#include <cstdint>

int main() {
    uint8_t* x1 = (uint8_t*)malloc(sizeof(uint8_t));
    *x1 = 10;
    // cout << "x1: " << (int)*x1 << endl;
    printf("x1: %d\n", *x1);
    // cout << "x1: " << (void*)x1 << endl;
    printf("x1: %#p\n", (void*)x1);
    free(x1);
}
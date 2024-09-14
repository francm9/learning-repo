#include <iostream>
#include <cstdint>

struct Test
{
    int x;
    int y;
};

typedef struct Test Test;


int main(){
    Test* t = (Test*)malloc(sizeof(Test));
    t->x = 10;
    t->y = 20;
    printf("t->x: %d\n", t->x);
    printf("t->y: %d\n", t->y);
    printf("t: %#p\n", (void*)t);
    free(t);

    Test t2;
    t2.x = 30;
    t2.y = 40;
    printf("t2.x: %d\n", t2.x);
    printf("t2.y: %d\n", t2.y);
    printf("t2: %#p\n", (void*)&t2);
    return 0;
}
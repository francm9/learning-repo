#include <iostream>
#include <cstdint>

class Test
{
public:
    void setX(int x);
    void setY(int y);
    int getX();
    int getY();
    Test(){
        this->x = 1;
        this->y = 1;
        arr = new int[10];
    }
    Test(int x, int y);
    ~Test(){            //Destructor -> Se ejecuta cuando se elimina el objeto
        delete[] arr;   //Liberar memoria -> No es necesario cuando se declara de forma estática
    }
private:
    int x;
    int y;
    int* arr;
};

void Test::setX(int x){
    this->x = x;
}

Test::Test(int x, int y){
    this->x = x;
    this->y = y;
    this->arr = (int*)malloc(sizeof(int) * 10);
}

void Test::setY(int y){
    this->y = y;
}

int Test::getX(){
    return this->x;
}

int Test::getY(){
    return this->y;
}

void sum1(int* a){
    *a = *a + 1;
}

using namespace std;

int main(){
    Test* t = new Test();       //No reserva la memoria hasta que se utiliza
    t->setX(10);
    t->setY(20);
    cout << "t->x: " << t->getX() << std::endl;
    cout << "t->y: " << t->getY() << std::endl;
    cout << "t: " << t << std::endl;
    delete t;

    Test t2;
    t2.setX(30);
    t2.setY(40);
    cout << "t2.x: " << t2.getX() << std::endl;
    cout << "t2.y: " << t2.getY() << std::endl;
    cout << "t2: " << &t2 << std::endl;

    Test t3;
    cout << "t3.x: " << t3.getX() << std::endl;
    cout << "t3.y: " << t3.getY() << std::endl;

    Test t4(50, 60);
    cout << "t4.x: " << t4.getX() << std::endl;
    cout << "t4.y: " << t4.getY() << std::endl;

    int a = t4.getX();
    sum1(&a);
    cout << "a: " << a << std::endl;
    return 0;
}
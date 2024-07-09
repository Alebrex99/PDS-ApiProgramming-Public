#include <iostream>

class Test {
public:
        Test() {
            std::cout<<"Costruito Test @"<<this<<std::endl;
        }
        ~Test() {
            std::cout<<"Distrutto Test @"<<this<<std::endl<<std::endl;
        }
};


int main ()

{

    std::cout<<"Main"<<std::endl;

    Test t;

    std::cout<<"Fine Main"<<std::endl;

};
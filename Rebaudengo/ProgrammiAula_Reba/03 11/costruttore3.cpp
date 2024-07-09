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

Test t1;

int main ()

{
    std::cout<<"Main"<<std::endl;

    for (int i =0 ; i<3 ; i++)
    {
        Test t;

        std::cout<<"interazione #"<<i<<std::endl;
    }

    std::cout<<"Fine Main"<<std::endl;

};
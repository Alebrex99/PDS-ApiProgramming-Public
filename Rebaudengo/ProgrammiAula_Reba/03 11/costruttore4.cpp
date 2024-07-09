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

    Test *ptr = new Test();
    for (int i =0 ; i<3 ; i++)
    {
        Test t;

        if (i==1) delete ptr;

        std::cout<<"interazione #"<<i<<std::endl;
    }

    std::cout<<"Fine Main"<<std::endl;

};
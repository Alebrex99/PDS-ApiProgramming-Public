#include <iostream>

class Test {
public:
        Test() {
            std::cout<<"Costruito Test @"<<this<<std::endl;
        }
        ~Test() {
            std::cout<<"Distrutto Test @"<<this<<std::endl;
        }
};

int main ()

{
    std::cout<<"Main"<<std::endl;
    for (int i =0 ; i<3 ; i++)
    {
        std::cout<<"interazione #"<<i<<std::endl;

        Test t;
        
    }

    std::cout<<"Fine Main"<<std::endl;

};
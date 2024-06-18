#include <iostream>

class Test{
public:
    Test(){
        std::cout<<"Un oggetto di tipo test è stato costruito all'indirizzo " <<this<<std::endl;
    }
    ~Test(){
        std::cout<<"Un oggetto di tipo test all'indirizzo " <<this<<" è stato distrutto."<<std::endl;
    }
};

int f() {
    std::cout << "Hello, World!" << std::endl;
    /*se creo un obj prima lo costruisc eprima di tutto*/
    Test t1;

    /*for (int i=0; i<5; i++){
        Test t;
        std::cout<<"---"<<std::endl;
        //in stack creo e distruggo oggetto per 5 volte
    }*/
    /*for finisce , ma finito anche il main e buttato via pure t1, il distruttore lo chiama
     * per me il compilatore
     * quando obj inizia ad esistere : COSTRUTTORE, cessa esistere : DISTRUTTORE*/

    for (int i=0; i<5; i++){
        Test t;
        //if (i==1) return 0;
        if(i==1) throw std::exception();
        std::cout<<"---"<<std::endl;
        /*in stack creo e distruggo oggetto per 5 volte*/
    }
    //sullo stack ho vari obj che tolgo e metto, qualunque sia ragione per cui esco, obj viene tolto

    /*con eccezione:
     * - crea t
     * crea t1
     * butta via, poi ad una certa con eccezione da problemi , siamo nel main non alternativa di risalita*/
    return 0;
}

int main(){
    try {
        f();
    }
    catch (int i){

    }
}

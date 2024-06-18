#include <iostream>

class Base {
public:
    virtual int m(){
        return 1;
    }
};

class Der1: public Base{
    int i;
public:
    int m(){
        return 2;
    }
};

class Der2: public Base{
    double f;
public:
    int m(){
        return 3;
    }
};


void print(Base* ptr){
    std::cout<<"Print: " <<ptr->m() <<std::endl;
}

int main() {
    std::cout << "Hello, World!" << std::endl;
    Base b;
    Der1 d1;
    Der2 d2;
    std::cout<<"Base: "<<sizeof b<<std::endl;
    std::cout<<"Der1: "<<sizeof d1<<std::endl;
    std::cout<<"Der2: "<<sizeof d2<<std::endl;
    print(&b); //li passo come puntatori perchè le classi potrebbero piu o meno vuota. ciascuna alloca 1 byte per identità sul this
    /*se alla print li passo un obj di tipo base, mi forzo ad avere 1 byte, non posso passare alle
     * altre funzioni, non ci stanno su spazio stack, mentre puntatori grandi uguali
     * der1: 4 byte
     * der2 . 8 byte (double)
     * in java sarebbe successo:
     * avrebbe syampato 1 2 3 , perche i metodi sono in automatico polimorfici :
     * - quando ho classe che definisce metodo e sottoclassi sovrascrivono override, in java viene chiamata versione specifica meotdo,
     *      se parto da rif generico : uso il generico
     *      se parto da puntatore a sottoclasse, uso metodo della sottoclasse
     * in C++ :
     * - se chiamo print che si aspetta puntatore a BASE, e passo puntatore a DER (posso), ma non è lecito
     * sapere che lui no nera un BASE, ma un DER, in C++ in classe s eho puntatore a qualcosa non so ca cosa punto, in java si (con getclass)
     * qui lo sa il compilatore che verifica sia ok codice, ma non si sa a run time.
     * in c++ puoi comportarti ocme java, ma necessario nin SUPERCLASSE davanti  ametodo polimorfico devi mettere virtual.
     * quindi quando avro puntatore a sta cosa e vedo invocato m , diro : m è quello chiamoto a puntatore a superclasse e torno 1, oppure
     * m chiamto come puntatore a sottoclasse?
     * vedo che stampa effettivamente 1 2 3.
     * NOTO CHE LE CLASI DIVENTANO PIU GRANDI.
     * es) Der1 : 8 byte -> 16 byte
     * mentre in java tutti i metodi sono polimorfici in natura, con cosa qualunque se chiamo to string viene chiamato to string di quella classe li, m
     * anche se non ho staticamente quelle classe li, (se in java passo object e invoca to string , stampa versione
     * propria di to string della classe, ma java spreca cosi).
     * C++ :
     * fa scegliere: SE VUOI USARE POLIMORIFISMO , MARCHI MTODI POLIMORIFIC CON VIRTUAL , funziona ma obj spreca memoria, diventa piu grosso.
     * se non vuoi usarlo , ok tiene conto spazio necessario, ma perche obj devon diventare piu grossi?
     * come fa a far in modo che 2 chimate identiche in apprenza:
     * print
     * a dare origine a 3 comportamenti distinti; print sa solo che ha ricevuto un BASE, ma che meotdo chiamo in realtà?
     * se di m ne conosce 3 tipi diversi?
     * */
    print(&d1);
    print(&d2);
    std::cout <<b.m()<< std::endl;
    std::cout <<d1.m()<< std::endl;
    return 0;
}

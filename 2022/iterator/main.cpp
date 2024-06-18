#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <list>
#include <set>
int main() {
    //MODIFICA
    //invece di definire v come vector di string lo definisco ocme list
    /*potrei dire allo stesso modo che non ho list ma vector , ma anche set o altro*/
    std::list<std::string> v;
    //std::vector<std::string> v;
    //std::set<std::string> v {"alfa", "beta", "gamma", "delta"};
    // ordine set lo sceglie lui : lui si tiene alfabetico in realtà, ma si popola diversamente

    v.push_back("alfa");
    v.push_back("beta");
    v.push_back("gamma");
    v.push_back("delta");

    std::string s = *std::max_element(v.begin(), v.end()); //#sterisco per vedere contenuto, se no torna indice
    std::cout<<"il più grande è "<<s<<std::endl;

    //per la lista
    //non potevi scrivere sort ne il FOR bechero, perche non ha comportamento come array con []
    /*estrae primo elemento in foreach , passando primo elemento, */
    std::for_each(v.begin(), v.end(), [](auto it){std::cout<<it<<std::endl;});


    //per il vector
    //std::sort(v.begin(), v.end());
    /* non posso fare sort su struttura senza quelle caratteristiche (come il SET) o una LISTA,
     * non implementa random acces iterator , ossia non puo imisurare distanza tr< elementi
     * mentre in vector si*/

    //FOR becero: base
    for (int i=0; i< v.size(); i++){
        std::cout<<i<<"<<v[i]"<<std::endl;
    }
    return 0;

}

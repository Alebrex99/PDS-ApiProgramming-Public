#include <iostream>
#include <iostream>
#include <thread>

int a = 0;	//Questo non è possibile in safe RUST
/*AFTER E BEFORE non sono soggetti a problems cache perche sono locali al thread.
 * ogni thread ha il suo proprio after.
 * after e before sono LOCALI.*/
void run() {
    while (a >= 0) {
        int before = a;
        a++;
        int after = a;
        if (after-before != 1)
            std::cout << before << " -> " << after
                      << "(" << after-before << ")\n";
    }
}

//creo due thread e ne attendo la terminazione
/*fotografia che faccio prima di a e foto che fai dopo : delta tempo che magari
 * implica presenza dell'altro.
 * i core usati da OS per i fatti suoi, possibile che 1 sospeso mentre gira l'altro
 * IL DELTA
 * */
int main() {
    std::thread t1(run);
    std::thread t2(run);

    t1.join();
    t2.join();
}

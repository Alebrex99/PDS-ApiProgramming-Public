#include <stdio.h>

int main() {
    typedef struct {
       int b ;
       int c ;
    }numeri, *ciao;
    int a=10;
    int* p;
    p=&a;
    printf("indirizzo di a: %d \n", p); //indirizzo a
    printf("valore di a: %d", *p);

    return 0;
}

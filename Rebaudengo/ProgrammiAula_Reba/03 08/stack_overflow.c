#include <stdio.h>

void func(int i)
{
    int array[1000*100];

    printf("%d\n", i);

    func(i+1);
    return;
}

int main() {
    int i = 0;
   
	func(i);
   	return 0;
}
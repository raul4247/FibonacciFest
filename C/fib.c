#include <stdio.h>

int main()
{
    int n, fib1 = 0, fib2 = 1;
    scanf("%d",&n);

    printf ("Hello HacktoberFest!\n");

    for (int i = 0; i < n; i++)
    {
        int tmp;
        printf ("%d ",fib1);

        tmp = fib1;
        fib1 = fib2;
        fib2 += tmp;
    }
    printf ("\n");

    return 0;
}
#include <iostream>

using namespace std;

int main()
{
    int n, fib1 = 0, fib2 = 1;
    cin >> n;

    cout << "Hello HacktoberFest!" << endl;

    for (int i = 0; i < n; i++)
    {
        int tmp;
        cout << fib1 << ' ';

        tmp = fib1;
        fib1 = fib2;
        fib2 += tmp;
    }
    cout << endl;

    return 0;
}
#include <bits/stdc++.h>
using namespace std;

int main(int argv, char *args[]) {
    mt19937 rnd(atoi(args[1]));
    if (rnd() % 5 == 0) {
        cout << "ERROR" << endl;
    } else {
        cout << "solve" << endl;
    }
}
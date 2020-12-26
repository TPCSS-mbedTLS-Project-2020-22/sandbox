#include<stdio.h>

// this c function will be called from rust
int func_in_c(int x) {
    printf("in c here: %d\n", x);
    x = x + 1;
    printf("in c here: %d\n", x);
    return x;
}


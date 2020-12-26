#include <stdio.h>


/********** can put these into a lib.h  ***********/
/* declare the function type */
extern int func_in_rust(int);

/*************************************************/

int main() {

    int x, y;

    x = 33;
    printf("in C: before calling rust: %d\n", x);

    y = func_in_rust(x);
    printf("in C: after calling rust: %d\n", y);

    return 0;
}


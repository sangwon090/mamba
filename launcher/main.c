#include <stdio.h>

long long mamba_main();
long long fibo(long long n);

int main(void) {
    long long result = mamba_main();
    printf("mamba exited with code %d.", result);

    return 0;
}
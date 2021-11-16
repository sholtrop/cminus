/* functions without returntype are errors. */

supersum(int x, int y) {
    return x + y;
}

int main(void) {
    int a;
    int b;
    a = 1;
    b = 10;
    writeinteger(supersum(a, b));
    return 0;
}
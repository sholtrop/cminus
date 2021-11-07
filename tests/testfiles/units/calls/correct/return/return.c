int supersum(int x, int y) {
    return x+y;
}

int main(void) {
    int a;
    int b;
    int c;
    a = 10;
    b = 20;
    c = 30;
    writeinteger(supersum(a, b));
    writeinteger(supersum(supersum(c, b), a));
    return 0;
}
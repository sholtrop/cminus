int supersum(int x, int y, int x) {
    return x + y;
}

int main(void) {
    int a;
    int b;
    a = 1;
    b = 10;
    writeinteger(supersum(a, b, b));
    return 0;
}
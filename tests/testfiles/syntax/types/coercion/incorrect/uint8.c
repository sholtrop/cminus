int main(void) {
    int a = 3;
    unsigned b = 4;
    uint8_t c = a;
    c = b;

    writeinteger(a);
    writeinteger(b);
    writeinteger(c);
    return 0;
}
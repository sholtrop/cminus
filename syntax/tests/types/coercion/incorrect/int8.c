int main(void) {
    uint8_t a = 3;
    int b = 3;
    unsigned c = 3;
    int8_t d = a;
    d = b;
    d = c;

    writeinteger(a);
    writeinteger(b);
    writeinteger(c);
    writeinteger(d);
    return 0;
}
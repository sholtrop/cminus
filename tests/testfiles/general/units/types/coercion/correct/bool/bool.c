int main(void) {
    int8_t a = 5;
    if (a)
        writeinteger(1);
    else
        writeinteger(0);

    uint8_t b = 7;
    if (b)
        writeinteger(1);
    else
        writeinteger(0);

    int c;
    c = 2;
    if (c)
        writeinteger(1);
    else
        writeinteger(0);

    unsigned d = 256;
    if (d)
        writeinteger(1);
    else
        writeinteger(0);

    return 0;
}


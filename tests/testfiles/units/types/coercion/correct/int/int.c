int main(void) {
    int a;
    a = 2 == 3;
    writeinteger(a); /* false, so 0 */
    int b;
    b = 3 > 2;
    /* true, so b > 0, and should write 1 */
    if (b > 0) {
        writeinteger(1);
    } else {
        writeinteger(0);
    }

    int8_t c = 3;
    int d;
    d = c;
    writeinteger(d); /* 3 */

    uint8_t e = 5;
    int f;
    f = e;
    writeinteger(f); /* 5 */

    return 0;
}
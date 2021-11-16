int main(void) {
    int8_t a = 2 == 3;
    writeinteger(a); /* false, so 0 */
    int8_t b = 3 > 2;
    /* true, so b > 0, and should write 1 */
    if (b > 0) {
        writeinteger(1);
    } else {
        writeinteger(0);
    }


    return 0;
}
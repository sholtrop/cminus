int main(void) {
    uint8_t a = 2 == 3;
    writeunsigned(a); /* false, so 0 */
    uint8_t b = 3 > 2;
    /* true, so b > 0, and should write 1 */
    if (b > 0) {
        writeunsigned(1);
    } else {
        writeunsigned(0);
    }

    int8_t c = 3;
    uint8_t d = c;
    writeunsigned(d); /* 3 */


    return 0;
}
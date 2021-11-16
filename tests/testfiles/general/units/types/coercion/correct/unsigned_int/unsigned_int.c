int main(void) {
    unsigned a = 2 == 3;
    writeunsigned(a); /* false, so 0 */
    unsigned b = 3 > 2;
    /* true, so b > 0, and should write 1 */
    if (b > 0) {
        writeunsigned(1);
    } else {
        writeunsigned(0);
    }

    int8_t c = 3;
    unsigned d = c;
    writeunsigned(d); /* 3 */

    uint8_t e = 5;
    unsigned f = e;
    writeunsigned(f); /* 5 */

    int g;
    g = 7;
    unsigned h = g;
    writeunsigned(h); /* 7 */


    return 0;
}
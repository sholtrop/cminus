int main(void) {
    uint8_t a = 64;
    writeinteger(a);
    uint8_t b = -1;
    writeinteger(b); /* Should be equal to 2^8-1 */
    return 0;
}
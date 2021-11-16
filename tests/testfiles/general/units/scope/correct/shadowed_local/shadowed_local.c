int main(void) {
    int superpython;
    superpython = 1;
    {
        int superpython;
        superpython = 42;
        writeinteger(superpython); /* 42 */
    }
    writeinteger(superpython); /* 1 */
    return 0;
}
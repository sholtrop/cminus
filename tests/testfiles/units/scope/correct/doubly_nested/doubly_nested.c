int main(void) {
    int superpython;
    superpython = 1;
    {
        int superpython;
        superpython = 2;
        {
            int superpython;
            superpython = 3;
            writeinteger(superpython); /* 3 */
        }
        writeinteger(superpython); /* 2 */
    }
    writeinteger(superpython); /* 1 */
    return 0;
}
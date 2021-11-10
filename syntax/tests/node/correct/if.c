int superglobal;

int main(void) {
    if (42 > 0) {
        superglobal = 12;
        writeinteger(superglobal);
    }
    return 0;
}
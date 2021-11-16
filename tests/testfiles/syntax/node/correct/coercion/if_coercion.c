int superglobal;

int main(void) {
    int a = 1;
    if (a) {
        superglobal = 12;
        writeinteger(superglobal);
    }
    return 0;
}
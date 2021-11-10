int superglobal;

int main(void) {
    if (42 > 0) {
        superglobal = 42;
    } else {
        superglobal = 1;
    }
    writeinteger(superglobal);
    return 0;
}
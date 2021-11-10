/* Expects 3 warnings about unused variables */

int main(void) {
    int a = 1;
    int b = 4; /* unused */
    int c = 2; /* unused */
    int d = 42;/* unused */
    writeinteger(a);
    return 0;
}
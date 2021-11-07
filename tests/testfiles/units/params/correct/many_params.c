/* >6 parameters to check the use of stack */
int add(int a, int b, int c, int d, int e, int f, int g, int h) {
    writeinteger(g); /* 2 */
    writeinteger(h); /* 3 */
    return a + b + c + d + e + f + g + h;
}

int main(void) {
    writeinteger(add(1, 1, 1, 1, 1, 1, 2, 3)); /* 11 */
    return 0;
}
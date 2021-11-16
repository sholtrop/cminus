int main(void) {
    int a;
    int b;
    a = 100;
    b = 100;

    while ((a + 1) > b) {
        writeinteger(b); /* should run for 1 time */
        b = b + 1;
    }
    return 0;
}
int main(void) {
    int a;
    int b;
    a = 100;
    b = 90;

    while (a > b) {
        writeinteger(b); /* should run for 10 times */
        b = b + 1;
    }

    while (a > b) {
        writeinteger(0); /* should run for 0 times */
        b = b + 1;
    }

    while ((a + 1) > b) {
        writeinteger(b); /* should run for 1 time */
        b = b + 1;
    }

    while (a > 1000) {
        writeinteger(0); /* should run for 0 times */
        a = a + 1;
    }
    return 0;
}
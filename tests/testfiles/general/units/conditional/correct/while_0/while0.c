int main(void) {
    int a;
    int b;
    a = 100;
    b = 90;

    while (b > a) {
        writeinteger(0); /* should run for 0 times */
        b = b + 1;
    }
    writeinteger(1);
    return 0;
}
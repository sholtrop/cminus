int main(void) {
    int a;
    a = 100;
    if (a > 10) {
        writeinteger(a); /* should land here */
    }

    if (a > 1000) {
        writeinteger(1000);
    }
    return 0;
}
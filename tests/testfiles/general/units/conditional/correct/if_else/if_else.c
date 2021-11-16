int main(void) {
    int a;
    a = 100;
    if (a > 10) {
        writeinteger(a); /* should land here */
    } else {
        writeinteger(0);
    }

    if (a < 10) {
        writeinteger(0);
    } else {
        writeinteger(a); /* should land here */
    }
    return 0;
}
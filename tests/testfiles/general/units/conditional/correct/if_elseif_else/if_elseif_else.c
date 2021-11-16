int main(void) {
    int a;
    a = 100;
    if (a > 1000) {
        writeinteger(1000);
    } else if (a > 99) {
        writeinteger(a); /* should land here */
    } else if (a > 10) {
        writeinteger(1);
    } else {
        writeinteger(0);
    }


    if (a > 99) {
        writeinteger(a); /* should land here */
    } else if (a > 55) {
        writeinteger(10);
    } else if (a > 10) {
        writeinteger(1);
    } else {
        writeinteger(0);
    }

    if (a > 1000) {
        writeinteger(1000);
    } else if (a > 500) {
        writeinteger(10);
    } else if (a > 242) {
        writeinteger(1);
    } else {
        writeinteger(a); /* should land here */
    }
    return 0;
}
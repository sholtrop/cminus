int readinteger(void) { /* Invalid: Should not redeclare built-in function name. */
    return 1;
}

int main(void) {
    int a;
    a = readinteger();
    writeinteger(a);
    return 0;
}
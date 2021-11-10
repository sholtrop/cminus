unsigned readunsigned(void) { /* Invalid: Should not redeclare built-in function name. */
    return 1;
}

int main(void) {
    unsigned a;
    a = readinteger();
    writeunsigned(a);
    return 0;
}
int else(int statement) { /* Invalid: Should not redeclare built-in function name. */
    return statement;
}

int main(void) {
    int a;
    a = else(42);
    writeinteger(a);
    return 0;
}
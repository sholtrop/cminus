int while(int statement) { /* Invalid: Should not redeclare built-in function name. */
    return statement;
}

int main(void) {
    int a;
    a = while(42);
    writeinteger(a);
    return 0;
}
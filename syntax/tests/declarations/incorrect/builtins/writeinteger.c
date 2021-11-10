void writeinteger(void) { /* Invalid: Should not redeclare built-in function name, even if it has a different signature (other parameters) */
    writeinteger(1);
}

int main(void) {
    writeinteger();
    return 0;
}
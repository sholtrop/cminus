void writeunsigned(void) { /* Invalid: Should not redeclare built-in function name, even if it has a different signature (other parameters) */
    writeunsigned(1);
}

int main(void) {
    writeunsigned();
    return 0;
}
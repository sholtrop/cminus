int supersum(int a, int b) {
    return a + b;
}

int main(void) {
    int supersum = 42;
    writeinteger(supersum(supersum, 9)); /* cannot call shadowed function. */
    return 0;
}
int main(void) {
    {
        int a = 42;
    }
    writeinteger(a); /* cannot reference out-of-scope variable. */
    return 0;
}
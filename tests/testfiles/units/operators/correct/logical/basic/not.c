int main(void) {
    int a;
    int b;
    int ans;

    a = 0;
    b = !a;


    writeinteger(b);

    ans = !!!a; /* odd number of not signs, so non-zero value, e.g. 1. */
    writeinteger(ans);

    ans = !ans; /* odd number of minus signs, was non-zero value, e.g. 1, so now 0 */
    writeinteger(ans);
    return 0;
}
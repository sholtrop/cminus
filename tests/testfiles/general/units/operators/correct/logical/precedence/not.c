int main(void) {
    int a;
    int b;
    int ans;

    a = 1;
    b = 0;

    ans = !b / a; /* !0 / 1 == non-zero, e.g. 1 / 1 = non-zero, e.g. 1 */
    writeinteger(ans);

    ans = a * !!!b; /* 1 * !!!0 = 1 * non-zero, e.g. 1 = non-zero, e.g. 1 */
    writeinteger(ans);

    ans = a % !b; /* 1 % !0 = 1 % non-zero, e.g. 1 = non-zero, e.g. 1 iff 'true' is defined as > 1, 0 iff 'true' is defined as exactly 1. */
    writeinteger(ans);
    return 0;
}
int main(void) {
    int a;
    int b;
    int ans;

    a = 0;
    b = !a;

    ans = a && b; /* 0 && non-zero, e.g. 1 = 0 */
    writeinteger(ans);

    ans = b && a; /* non-zero, e.g. 1 && 0 = 0 */
    writeinteger(ans);

    ans = !(b && b); /* !(non-zero, e.g. 1 && non-zero, e.g. 1) = !(non-zero, e.g. 1) = 0 */
    writeinteger(ans);

    ans = a && a; /* 0 && 0 == 0 */
    writeinteger(ans);
    return 0;
}
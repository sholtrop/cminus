int main(void) {
    int a;
    int b;
    int ans;

    a = 1;
    b = 2;

    ans = a > b; /* 1 > 2 = 0 */
    writeinteger(ans);

    ans = b > a; /* 2 > 1 = non-zero, e.g. 1 */
    writeinteger(ans);

    ans = b > b; /* 2 > 2 = 0 */
    writeinteger(ans);
    return 0;
}
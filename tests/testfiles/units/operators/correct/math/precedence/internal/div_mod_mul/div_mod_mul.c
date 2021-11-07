int main(void) {
    int a;
    int b;
    int c;
    int d;
    int ans;

    a = 100;
    b = 2;
    c = 3;
    d = 8;

    ans = a % d / b; /* 100 % 8 / 2 = 4 / 2 = 2 */
    writeinteger(ans);

    ans = a % c * d; /* 100 % 3 * 8 = 1 * 8 = 8 */
    writeinteger(ans);

    ans = a * c % d / c; /* 100 * 3 % 8 / 3 = 300 % 8 / 3 = 4 / 3 = 1 */
    writeinteger(ans);

    return 0;
}
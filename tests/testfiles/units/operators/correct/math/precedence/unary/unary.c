int main(void) {
    int a;
    int b;
    int c;
    int d;
    int ans;

    a = -100;
    b = 100;
    c = 3;
    d = 8;

    ans = +a % d; /* +-100 % 8 = -4 */
    writeinteger(ans);

    ans = -b / c; /* -100 / 3 = -33.333 ---> -33 */
    writeinteger(ans);

    ans = d * --+-c; /* 8 * --+-3 = 8 * -3 = -24 */
    writeinteger(ans);
    return 0;
}
int main(void) {
    int a;
    int b;
    int c;
    int ans;

    a = 1;
    b = 2;
    c = 0;

    ans = c <= c == c; /* 0 <= 0 == 0 = non-zero, e.g. 1 == 0 = 0 */
    writeinteger(ans);

    ans = b >= c == c; /* 2 >= 0 == 0 = non-zero, e.g. 1 == 0 = 0 */
    writeinteger(ans);

    ans = c > c == c; /* 0 > 0 == 0 = 0 == 0 = non-zero, e.g. 1 */
    writeinteger(ans);

    ans = c == c < c; /* 0 == 0 < 0 = 0 == 0 = non-zero, e.g. 1 */
    writeinteger(ans);

    ans = c <= a != a; /* 0 <= 1 != 1 = non-zero, e.g. 1 != 1 = 0 */
    writeinteger(ans);
    return 0;
}
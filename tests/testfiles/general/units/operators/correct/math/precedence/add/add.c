int main(void) {
    int a;
    int b;
    int c;
    int d;
    int ans;

    a = 1;
    b = 2;
    c = 3;
    d = 4;

    ans = b + c < d; /* 2 + 3 < 4 = 5 < 4 = 0 */
    writeinteger(ans);

    ans = b + a + a + a <= d; /* 2 + 1 + 1 + 1 <= 4 = 5 <= 4 = 0 */
    writeinteger(ans);
    return 0;
}
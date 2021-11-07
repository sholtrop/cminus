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

    ans = c + a / d; /* 3 + 100 / 8 = 3 + 12.5 --> 3 + 12 = 15 */
    writeinteger(ans);

    ans = b - d / b; /* 2 - 8 / 2 = 2 - 4 = -2 */
    writeinteger(ans);
    return 0;
}
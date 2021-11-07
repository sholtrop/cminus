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

    ans = c + a % d; /* 3 + 100 % 8 = 3 + 4 = 7 */
    writeinteger(ans);

    ans = b - d % c; /* 2 - 8 % 3 = 2 - 2 = 0 */
    writeinteger(ans);
    return 0;
}
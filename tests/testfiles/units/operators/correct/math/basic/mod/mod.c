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

    ans = a % b; /* 0 */
    writeinteger(ans);
    ans = a % c; /* 1 */
    writeinteger(ans);
    ans = a % d % c; /* 100 % 8 % 3 = 4 % 3 = 1 */
    writeinteger(ans);
    return 0;
}
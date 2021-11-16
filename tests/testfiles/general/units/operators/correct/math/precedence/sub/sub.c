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

    ans = b - d > c; /* 2 - 4 > 3 = -2 > 3 = 0 */
    writeinteger(ans);

    ans = d - a - a >= c; /* 4 - 1 - 1 >= 3 = 2 >= 3 = 0 */
    writeinteger(ans);
    return 0;
}
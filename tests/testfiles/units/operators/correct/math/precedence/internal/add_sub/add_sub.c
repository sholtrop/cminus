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

    ans = d + b - c; /* 8 + 2 - 3 = 10 - 3 = 7 */
    writeinteger(ans);

    ans = d - b + a; /* 8 - 2 + 100 = 6 + 100 = 106 */
    writeinteger(ans);
    return 0;
}
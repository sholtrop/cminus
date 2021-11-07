int main(void) {
    int a;
    int b;
    int c;
    int ans;

    a = 100;
    b = 2;
    c = 4;

    ans = a / c; /* 100 / 4 = 25 */
    writeinteger(ans);
    ans = a / c / b; /* 100 / 4 / 2 = 25 / 2 = 12.5 --> 12 */
    writeinteger(ans);
    return 0;
}
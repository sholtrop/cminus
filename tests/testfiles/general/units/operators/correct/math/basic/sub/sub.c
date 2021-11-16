int main(void) {
    int a;
    int b;
    int c;
    int d;
    int ans;

    a = 100;
    b = 2;
    c = 3;
    d = 4;
    ans = a - d; /* 96 */
    writeinteger(ans);

    ans = b - d; /* -2 */
    writeinteger(ans);

    ans = a - b - c - d ; /* 91 */
    writeinteger(ans);
    return 0;
}
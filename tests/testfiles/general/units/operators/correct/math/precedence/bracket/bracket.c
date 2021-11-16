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

    ans = a - b - c - d; /* 91 */
    writeinteger(ans);
    ans = a - b - (c - d); /* 99 */
    writeinteger(ans);
    return 0;
}
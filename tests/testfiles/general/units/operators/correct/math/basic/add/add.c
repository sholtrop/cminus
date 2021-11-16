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
    ans = a + b + ((c + d) + 5 + 6); /* 21 */
    writeinteger(ans);
    return 0;
}
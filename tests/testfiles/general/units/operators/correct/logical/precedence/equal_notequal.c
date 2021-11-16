int main(void) {
    int a;
    int b;
    int ans;

    a = 0;
    b = !a;

    ans = a == a && b == a; /* 0 == 0 && non-zero, e.g. 1 == 0 = non-zero, e.g. 1 && non-zero, e.g. 1 == 0 = non-zero, e.g. 1 && 0 = 0 */
    writeinteger(ans);

    ans = a == b || b == a; /* 0 == non-zero, e.g. 1 || non-zero, e.g. 1 == 0 = 0 || 0 = 0 */
    writeinteger(ans);

    ans = b != a || b; /*non-zero, e.g. 1 != 0 || non-zero, e.g. 1 = non-zero, e.g. 1 == non-zero, e.g. 1 = non-zero, e.g. 1 */
    writeinteger(ans);

    ans = b != a && a != b; /*non-zero, e.g. 1 != 0 && 0 != non-zero, e.g. 1 = non-zero, e.g. 1 && non-zero, e.g. 1 = non-zero, e.g. 1 */
    writeinteger(ans);
    return 0;
}
int main(void) {
    int a;
    int ans;

    a = 100;

    ans = -a; /* -100 */
    writeinteger(ans);

    ans = +ans; /* -100 */
    writeinteger(ans);

    ans = ---------ans; /* even number of minus signs, so 100 */
    writeinteger(ans);

    ans = ++-------+++ans; /* odd number of minus signs, so -100 */
    writeinteger(ans);

    ans = ++++++--+ans; /* even number of minus signs, so stays -100 */
    writeinteger(ans);
    return 0;
}
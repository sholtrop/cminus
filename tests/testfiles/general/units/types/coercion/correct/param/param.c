int smallfunc(int x) {
    return x + 14;
}

int main(void) {
    int a;
    a = -1;
    int ans;
    ans = smallfunc(a);
    writeinteger(ans);
    return 0;
}
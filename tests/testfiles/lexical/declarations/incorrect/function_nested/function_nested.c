int main(void) {
    int x, y;
    x = 10;
    y = 1;
    int max(int a, int b) {
        if (a > b)
            return a;
        return b;
    }

    writeinteger(max(x, y));
    return 0;
}
int oof(int x, y) {
    if (x > y)
        return y;
    return x;
}

int main(void) {
    oof(100, 1)(2, 4);
    return 0;
}
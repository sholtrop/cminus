uint8_t smallfunc(uint8_t x) {
    return x + 14;
}

int main(void) {
    int a = -1;
    uint8_t ans = smallfunc(a); /* cannot downcast an int to uint8_t */
    writeinteger(ans);
    return 0;
}


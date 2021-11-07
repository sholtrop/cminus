uint8_t smallfunc(uint8_t x) {
    return x + 14;
}

int main(void) {
    uint8_t a = 14;
    unsigned ans = smallfunc(a);
    writeunsigned(ans);
    return 0;
}
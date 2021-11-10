void smallfunc(uint8_t x) {
    return x + 14;
}

int main(void) {
    uint8_t a = 14;
    smallfunc(a);
    writeunsigned(a);
    return 0;
}
unsigned justadd(unsigned x) {
    return x + 14;
}

int main(void) {
    unsigned a; /* uninitialized */
    unsigned ans;
    ans = justadd(a);
    writeunsigned(ans);
    return 0;
}
int superglobal;

void t1(void) {
    return;
}

int t2(void) {
    return superglobal;
}
int main(void) {
    superglobal = 42;
    t1();
    t2();
    return 0;
}
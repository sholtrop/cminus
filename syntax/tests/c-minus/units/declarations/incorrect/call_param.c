
void func(int param, int x) {
    param(x);
}

int main(void) {
    int oof = 10;
    func(oof, 9);
    return 0;
}
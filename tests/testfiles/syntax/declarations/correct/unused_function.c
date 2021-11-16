/* Expects 2 warnings about unused functions */

int not_used(int x, int y, int z) {
   return x+y+z;
}

void no_use(void) {
    writeinteger(44);
}

void yeethon_python(int p, int y, int t, int h, int o, int n) {
    writeinteger(p+y+t+h+o+n);
}

int main(void) {
    int a = 1;
    int b = 10;
    int c = 99;
    int d = 3;
    int e = 50;
    int f = -100;
    yeethon_python(a, b, c, d, e, f);
    return 0;
}
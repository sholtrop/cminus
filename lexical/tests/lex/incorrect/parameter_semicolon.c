int max(int a, int b) {
    if (a > b)
        return a;
    return b;
}

void my_favourite_grade(int a; int b; int c) { // <-- semicolons instead of comma
    writeinteger(max(max(a, b), c));
}
int main(void) {
    my_favourite_grade(1, 10, 8);
    return 0;
}
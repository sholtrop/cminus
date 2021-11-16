int large_array[100];

void func(int idx) {
    writeinteger(large_array[idx]);
}

int main(void) {
    large_array[8] = 42;
    func(8);
    return 0;
}
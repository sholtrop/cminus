void print_arr(int arr[], int idx) {
    writeinteger(arr[idx]);
}

int main(void) {
    int large_array[100];
    large_array[99] = 1024;
    print_arr(large_array, 99);
    return 0;
}
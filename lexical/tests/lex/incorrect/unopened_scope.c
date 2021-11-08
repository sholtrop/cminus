int main(void) {
    if (10)                      // <-- Missing scope open
        writeinteger(1);
    }

    return 0;
}
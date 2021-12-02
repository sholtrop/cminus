int recurser(int x)
{
    if (x < 3)
    {
        writeinteger(x);
        return recurser(x + 1);
    }
    return x;
}

int main(void)
{
    int a;
    a = recurser(0);
    writeinteger(a);
    return 0;
}
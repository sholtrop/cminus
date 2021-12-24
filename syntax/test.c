void hello(int a)
{
    if (a == 0)
        return;
    else
        writeinteger(a);
}

int main(void)
{
    int x = 2;
    while (x > 0)
    {
        x = x - 1;
    }
    hello(x);
    return 0;
}
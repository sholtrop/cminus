int something(void)
{
    int x = 0;
}

int main(void)
{
    int x = 1 + 2 + 3 + 4;
    if (x > 0)
    {
        x = 1;
    }
    else
    {
        x = 2;
    }
    return x + something();
}
int something_else(void)
{
    return 1;
}

int something(void)
{
    something_else();
}

int main(void)
{
    return something();
}


int main(void)
{
    int v = 1;
    int z = v + 1;
    int x = z * v;
    int y = x * 2;
    int w = x + z * y;
    int u = z + 2;
    v = u + w + y;
    return v * u;
}
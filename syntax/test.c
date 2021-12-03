/* Program to select selection sort on a 4-element array. */

int x[4];

int minloc(int a[], int low, int high)
{
    int min_idx;
    min_idx = low;
    int min;
    min = a[low];
    int i;
    i = low + 1;
    while (i < high)
    {
        if (a[i] < min)
        {
            min = a[i];
            min_idx = i;
        }
        i = i + 1;
    }
    return min_idx;
}

void sort(int a[], int low, int high)
{
    int i;
    i = low;

    while (i < high - 1)
    {
        int tmp;
        int sort_min_idx;
        sort_min_idx = minloc(a, i, high);
        tmp = a[sort_min_idx];
        a[sort_min_idx] = a[i];
        a[i] = tmp;
        i = i + 1;
    }
}

int main(void)
{
    int i;
    i = 0;
    while (i < 4)
    {
        x[i] = readinteger();
        i = i + 1;
    }
    sort(x, 0, 4);
    i = 0;
    while (i < 4)
    {
        writeinteger(x[i]);
        i = i + 1;
    }
    return 0;
}
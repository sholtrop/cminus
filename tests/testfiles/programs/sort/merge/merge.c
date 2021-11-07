/* Program to mergesort 4 integers. */

int sorted[4];

void merge(int list[], int left, int mid, int right) {
    int i;
    int j;
    int k;
    int l;
    int ta;
    int tb;

    i = left;
    j = mid + 1;
    k = left;
    ta = 1;
    tb = 1;

    while (ta * tb) {
        if (list[i] <= list[j]) {
            sorted[k] = list[i];
            k = k + 1;
            i = i + 1;
        } else {
            sorted[k] = list[j];
            k = k + 1;
            j = j + 1;
        }
        if (i <= mid)
            ta = 1;
        else
            ta = 0;
        if (j <= right)
            tb = 1;
        else
            tb = 0;
    }

    if (i > mid) {
        l = j;
        while (l <= right) {
            sorted[k] = list[l];
            k = k + 1;
            l = l + 1;
        }
    } else {
        l = i;
        while (l <= mid) {
            sorted[k] = list[l];
            k = k + 1;
            l = l + 1;
        }
    }

    l = left;
    while (l <= right) {
        list[l] = sorted[l];
        l = l + 1;
    }
}

void mergesort(int list[], int left, int right) {
    if (left < right) {
        int mid;
        mid = left + right;
        mid = mid / 2;
        mergesort(list, left, mid);
        mergesort(list, mid + 1, right);
        merge(list, left, mid, right);
    }
}

int main(void) {
    int l[4];
    int i;
    i = 0;
    int n;
    n = 4;

    while (i < n) {
        l[i] = readinteger();
        i = i + 1;
    }

    mergesort(l, 0, n - 1);

    i = 0;
    while (i < n) {
        writeinteger(l[i]);
        i = i + 1;
    }
    return 0;
}
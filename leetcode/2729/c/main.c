bool isFascinating(int n) {
    int a = n * 2;
    int b = n * 3;

    if (a >= 1000 || b >= 1000) {
        return false;
    }

    int digits[10];

    // Count digits
    for (int i = 0; i < 3; i++) {
        int digit = n % 10;
        digits[digit]++;
        n /= 10;

        digit = a % 10;
        digits[digit]++;
        a /= 10;

        digit = b % 10;
        digits[digit]++;
        b /= 10;
    }


    if (digits[0] != 0) { return false; }
    for (int i = 1; i < 10; i++) {
        if (digits[i] != 1) { return false; }
    }

    return true;
}

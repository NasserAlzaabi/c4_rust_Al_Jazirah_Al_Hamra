int add(int a, int b) {
    return a + b;
}

int main() {
    int i = 0;
    int sum = 0;

    // While loop
    while (i < 5) {
        sum = add(sum, i); // Function call
        i = i + 1;
    }

    // If-else
    if (sum > 10) {
        printf("Sum is greater than 10: %d\n", sum);
    } else {
        printf("Sum is less than or equal to 10: %d\n", sum);
    }

    return 0;
}
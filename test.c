// Function with integer parameters
int add(int a, int b) {
    return a + b;
}

// Function with char parameter that returns its ASCII value
int get_ascii(char c) {
    return c; // returns the ASCII value automatically
}

// Function with string parameter (prints address, not content)
void print_addr(char *s) {
    // Just print the address value (as integer)
    printf("%d\n", s);
}

// Function with if-else
int check_positive(int num) {
    if (num > 0) {
        return 1;
    } else {
        return 0;
    }
}

// Main function with all required elements
int main() {
    // Integer variables
    int x = 10;
    int y = 20;
    int sum;
    
    // Char variable
    char c = 65; // ASCII for 'A'
    
    // String variable (char *)
    char *message = "Hello"; // String literal (won't dereference)
    
    // Print integer values with %d
    printf("%d\n", x);
    printf("%d\n", y);
    
    // Function call with parameters
    sum = add(x, y);
    printf("%d\n", sum);
    
    // Print ASCII value of character
    printf("%d\n", c);
    printf("%d\n", get_ascii(c));
    
    // Print string address (not content)
    printf("%d\n", message);
    print_addr(message);
    
    // While loop
    int count = 5;
    while (count > 0) {
        printf("%d\n", count);
        count = count - 1;
    }
    
    // If-else statement
    if (x > y) {
        printf("%d\n", 1);
    } else {
        printf("%d\n", 0);
    }
    
    return 0;
}
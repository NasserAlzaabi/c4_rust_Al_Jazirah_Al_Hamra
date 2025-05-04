// file with testing for all implemented features

// Function with integer parameters
int add(int a, int b) {
    return a + b;
}

int get_ascii(char c) {
    return c; // returns the ASCII value automatically
}

void print_addr(char *s) {
    // Just print the address value (as integer)
    printf("String address: %d\n", s);
}

// Function with if-else
int check_positive(int num) {
    if (num > 0) {
        return 1;
    } else {
        return 0;
    }
}

int main() {
    // Integer variables
    int x = 10;
    int y = 20;
    int sum;
    
    // Char variable
    char c = 'A'; // ASCII for 'A'
    
    // String variable
    char *message = "Hello"; // String literal (won't dereference)
    
    // Print integer values with %d
    printf("Value of x: %d\n", x);
    printf("Value of y: %d\n", y);
    
    // Function call with parameters
    sum = add(x, y);
    printf("Sum x,y: %d\n", sum);
    
    // Print ASCII value of character
    printf("Character c as int: %d\n", c);
    printf("ASCII value from get_ascii(): %d\n", get_ascii(c)); //passing char to func
    
    // Print string address
    print_addr(message);
    
    // While loop
    int count = 5;
    while (count > 0) {
        printf("Count down: %d\n", count);
        count = count - 1;
    }
    
    // If-else statement
    if (x > y) {
        printf("Is x > y: %d\n", 1);
    } else {
        printf("Is x > y: %d\n", 0);
    }
    
    return 0;
}
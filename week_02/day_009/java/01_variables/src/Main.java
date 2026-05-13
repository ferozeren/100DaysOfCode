public class Main {

    public static void main(String[] args) {
        // Variables, a reusable container for a value

        // Primitive, a simple value stored directly in memory (stack)
        // Reference, memory address (stack) that points to the (heap)

        // Primitive    Reference
        // ---------    ---------
        // int          string
        // double       array
        // char         object
        // boolean

        // Steps to create a variable
        // 1. Declaration
        // 2. Assignments

        int whole_number; // Declaration

        whole_number = 11; // Assignment

        // | Specifier | Data Type         | Example Output               |
        // |===========|-------------------|------------------------------|
        // |   `%s     | string            | "hello world"                |
        // |   `%d`    | integer (decimal) | 123                          |
        // |   `%f`    | floating-point    | 10.500000                    |
        // |   `%x`    | hexadecimal       | ff                           |
        // |   `%n`    | newline           | platform-specific line break |

        //Advanced Syntax: The full syntax for a placeholder is:
        // %[argument_index$][flags][width][.precision]conversion

        // Precision (.2f) :Controls decimal places. For example, `%.2f`
        // rounds a number to two decimal places (e.g., `3.14`).
        //
        // Width (%10s)**: Sets the minimum number of characters. If the value
        // is shorter, it is padded with spaces.
        //
        // Flags**:
        // `-` : Left-justifies the text within the specified width.
        // `,` : Adds a thousands separator (e.g., `1,000,000`).
        // `0` : Pads numbers with leading zeros instead of spaces

        System.out.println(String.format("Variable: $%d", whole_number)); // Printing variable

        float thousand = whole_number + 10000;

        System.out.println(String.format("Thousand: $%,.2f", thousand));
    }
}

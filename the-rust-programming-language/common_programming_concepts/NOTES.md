# Notes
## Variables and Mutability
- By default a variable is immutable. => increased safety
- Adding "mut" ahead of a varible make it mutable => example: `let mut x: u32 = 5;`
- Declaring constants using the const keyword instead of the let keyword => example: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`
- Constants aren’t just immutable by default—they’re always immutable.
- Shadowing allow us to apply a new value to a let variable by using another let with the same variable name => it overrides the old value

## Data Types
- Two data type subsets: scalar and compound
- When trying to parse a string, we need to indicate the target type like that:
```
let guess: u32 = "42".parse().expect("Not a number!");

// If we don’t add the : u32 type annotation shown in the preceding code, Rust will display an error

```
- A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages.
- Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where n is the number of bits that variant uses. So, an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127. Unsigned variants can store numbers from 0 to 2n − 1, so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.
- Additionally, the isize and usize types depend on the architecture of the computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
- Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
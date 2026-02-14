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

### Integer types
An integer is a number without a fractional component.
Rust provides two types of integer: 
- signed (example: i8)
- unsigned (example: u8)

We can use any of these variants to declare the type of an integer value.


| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit |i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| Architecture-dependent | isize | usize |


### Floating-Point Types
Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.

Example: `let x: f32 = 2.0`

### Numerical Operations
Rust supports the basic mathematical operations you’d expect for all the number types: 
- addition => `let sum = 5 + 10;`
- subtraction => `let difference = 95.5 - 4.3;`
- multiplication => `let product = 4 * 30;`
- division => `let quotient = 56.7 / 32.2; let truncated = -5 / 3; // Results in -1`
- remainder => `let remainder = 43 % 5;`

### The Boolean Type
As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size. The Boolean type in Rust is specified using bool.

### The Character Type
Rust’s char type is the language’s most primitive alphabetic type.

Example: `let character: char = 'C';`

Note that we specify char literals with single quotation marks, as opposed to string literals, which use double quotation marks. Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust.

### Compound Types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

**The Tuple Type**
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: Once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

`let tup: (i32, f64, u8) = (500, 6.4, 1);`

Is it possible to accessible to only one value of the tuple in two ways:
Destructuring: 
```
let tup: (i32, f64, u8) = (500, 6.4, 1);

let (x, y, z) = tup;
```

Period mehod:
```
let tup: (i32, f64, u8) = (500, 6.4, 1);

let first_value = tup.0; // 500

```



**The Array Type**
Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

Rust provide two types of array:
- Array => Stored in the stack with a fixed size
- Vector => Stored in the heap with a flexbile size

We write the values in an array as a comma-separated list inside square brackets:  

`let a = [1, 2, 3, 4, 5];`

It's possible to declare the type of each elements of an array like that:

`let a: [i32; 5] = [1, 2, 3, 4, 5];`

Also, it's possible to defined the same value for all elements like that:

`let a = [3; 5]; // Mean an array of length five with all values equals three let a = [3, 3, 3, 3, 3]`

**Array Element Access**

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:

```
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];
```



# Notes

## What Is Ownership ?
- Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running.

### The Stack and the Heap
**The stack** stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out (LIFO).
Adding data is called pushing onto the stack, and removing data is called popping off the stack. All data stored on the stack must have a known, fixed size.

**The heap** is less organized: When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.  
Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there.

### Ownership Rules
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable Scope
```
fn main()     {        // s is not valid here, since it's not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}    

```

- When s comes into scope, it is valid.
- It remains valid until it goes out of scope


### The String Type / Memory and Allocation

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

The memory must be requested from the memory allocator at runtime.
We need a way of returning this memory to the allocator when we’re done with our String.  
That first part is done by us: When we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

```
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid

```


There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.


### Variables and Data Interacting with Move

Multiple variables can interact with the same data in different ways in Rust.

```
    let x = 5;
    let y = x;
```

Here, x and y have the same value because integer have a known fixed size and pushed into the **stack**.

## References and Borrowing
Ampersand sign `&` used before a variable name means that's a reference to the following variable  

Example
```
fn main() {
    let s1 = String::from("hello"); // s1 is a pointer stored in the stack owned the "hello" string that is stored in the heap
    println!("{s1}");

    let len: usize = calculate_length(&s1); // s is a pointer to a reference of s1 (using ampersand), like that, s1 variable is not dropped after using it.
    println!("{len}");
}

fn calculate_length(s: &String) -> usize {
    return s.len(); 
}
```

**Note:** The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.

### Mutable References

Updating a borrowed value is possible with _mutable reference_.

A mutable reference can be declared like that:

```
fn main() {
    let mut s = String::from("hello");
    update_string(&mut s); // take a mutable reference as argument

    println!("{s}"); // hello my friend
}

fn update_string(string: &mut String) { // take a mutable reference as parameter
    string.push_str(" my friend")
}

```

**Mutable references have one big restriction:** If you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail.

This restricition prevent _**data race**_.

_**A data race**_ is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:  
```
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;

```



**Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references is in the println!, before the mutable reference is introduced**


## Summary
The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages. But having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book. Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.




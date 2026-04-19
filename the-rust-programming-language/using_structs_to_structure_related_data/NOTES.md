# Notes

## Defining and Instatiating Structs
To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together.  

**Example**
```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

```

- Access value with dot annotation => user.email
- shorthand assignement using the same field name 
```
fn set_user(email: String, name: String) -> User {
    User {
        email, // shorthand for email: email
        name,
        age: 25
    }
}
```
- Remain values operator `..`
```
let user2 = User {
    email: "test@hotmail.com",
    ..user1 // name and age from user 1
}
```

## Creating Different Types with Tuple Structs

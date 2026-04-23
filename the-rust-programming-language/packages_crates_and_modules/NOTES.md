# Notes
## Code's organization features
Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and crates

A **crate** is the smallest amount of code that the Rust compiler considers at a time.
Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.

A crate can come in one of two forms: a _binary_ crate or a _library_ crate. 
- **_Binary_** crates are programs you can compile to an executable that you can run, such as a command line program or a server. Each must have a function called `main`.  
**This is a binary root file: `src/main.rs`**

- **_Library_** crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate.  
**This is a library root file: `src/lib.rs`**

---

A **package** is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command line tool you’ve been using to build your code.

A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.


## Control Scope and Privacy with Modules
### Modules Cheat Sheet
This is how rust compiler looks the code:

- **_Start from the crate root:_** When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate and src/main.rs for a binary crate) for code to compile.
- **_Declaring modules:_** In the crate root file, you can declare new modules; say you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
Inline, within curly brackets that replace the semicolon following mod garden
In the file src/garden.rs
In the file src/garden/mod.rs
- **_Declaring submodules:_** In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
Inline, directly following mod vegetables, within curly brackets instead of the semicolon
In the file src/garden/vegetables.rs
In the file src/garden/vegetables/mod.rs
- **_Paths to code in modules:_** Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
- **_Private vs. public:_** Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
- **_The use keyword:_** Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus;, and from then on you only need to write Asparagus to make use of that type in the scope.


### Grouping Related Code in Modules
**_Modules_** let us organize code within a crate for readability and easy reuse. Modules also allow us to control the privacy of items because code within a module is private by default.

A module library can be initialized with the flag `--lib`.  
Example: `cargo new math --lib`

Then a library is created with the following structure:

```
maths
 └── operations
     ├── add
     │   ├── add_integer
     │   └── add_floating
     └── multiply
         ├── multiply_integer
         └── multiply_floating
```

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

## Paths for Referring to an Item in the Module Tree
A path can take two forms:

- An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
- A relative path starts from the current module and uses self, super, or an identifier in the current module.

Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. To continue with our metaphor, think of the privacy rules as being like the back office of a restaurant: What goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant they operate.

Rust chose to have the module system function this way so that hiding inner implementation details is the default. That way, you know which parts of the inner code you can change without breaking the outer code. However, Rust does give you the option to expose inner parts of child modules’ code to outer ancestor modules by using the pub keyword to make an item public.

### Exposing Paths with the pub Keyword
Module childs are private by default so if you want to make it public and be able to compie your code, you need to add the `pub` keyword in the function/module's name.  
```
mod operations {
    mod add {
        // private
        fn add_integer() {}

        // public
        pub fn public_add_integer() {}
    }
}
```

### Starting Relative Paths with super
Using super allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier when the module is closely related to the parent but the parent might be moved elsewhere in the module tree someday.

Usage:
```
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

### Making Structs and Enums Public
We can also use pub to designate structs and enums as public, but there are a few extra details to the usage of pub with structs and enums. If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis. 

It's also possible to make all fields of en enum public by adding the `pub` keyword in the enum declaration.

```
pub enum Appetizer {
    Soup,
    Salad,
}
```

Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

There’s one more situation involving pub that we haven’t covered, and that is our last module system feature: the use keyword. We’ll cover use by itself first, and then we’ll show how to combine pub and use.

# Objectives of the exercise

The goal of this live coding session is to reimplement a simple binary (`yes`) in its
simplest form.
## Simplest simple form of yes

When invoked without any arguments, `yes` simply prints the string "y" on stdout over
and over until the program is terminated.

In order to do this, let's create a function `get_string()` that will return the correct
string to print. Then, let's implement a `show_string()` function to display it on
stdout. Obviously, this is much more complex than it needs to be. We could do all of
that in the main function. But we just learned about functions so let's use them!

## Unit testing yes

We haven't yet seen the syntax for unit testing, so we'll explore it now. For now,
the tests should be really simple and basically make sure that `get_string()` returns
the correct string

## Getting command line arguments

Calling `yes` with an argument will cause the program to print that argument over and
over instead of the string "y". In C, we would implement it by doing this:
```c
int main(int argc, char **argv) {
    // 0 extra arguments provided - Remember, argc starts at 1
    if (argc < 2) {
        while (1) {
            printf("y\n");
        }
    } else {
        while (1) {
            printf("%s\n", argv[1]);
        }
    }
}
```

In Rust, you will need to use the `std::env::args()` function.

The function `get_string()` will need to be modified. Your unit tests should reflect
that!

## Going further - Discovering crates and clap

Using the clap crate, parse arguments given to your program and display something useful
when using the `-h|--help` option. The program should contain an author, a version
and a small description!

## Going further - Splitting the code in modules

Split the program in three files: `main.rs`, `args.rs` and `printer.rs`. Using `mod` and
`use` directives, create a good looking architecture!

## Going further - Unit testing stdout

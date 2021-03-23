# Welcome to the Zoo !

Oh my god, the apes are running wild, some SRS students are in the mix, we need someone
to come in and take care of this mess. Your mission, if you accept it, will be to
implement a functional, clean and modular zoo. You will need to handle the animals, the
personel as well as the infrastructure. And just like any working zoo, it will need to
be unit tested.

## 1 - Creating some animals

Create a new, empty project. Feel free to create a new branch on your fork so you can ask
us to review it later.

Quick reminder in case you forgot:

```sh
> git checkout -b zoo_live_coding
> git push -u origin zoo_live_coding # To push your branch
> cargo new zoo # Put it wherever you want!
```

Let's create a few animals and implement their behaviors. For example, apes should scream
and skunks should stink. You can add as many animals as you want, but for now we'll focus
on Apes, GISTREs and Skunks. Remember to put all those animals in the same module so our
zoo stays ordered.

### Skunks

#### Fields

#### Methods

- `stink(self)`

### Apes

#### Fields

- `banana_counter: u32`
- `happy: bool` An ape isn't happy once it runs out of bananas

#### Methods

- `population() -> u32` There are very few apes remaining...
- `scream(self)`
- `eat_banana(self)`

### GISTREs

You need to create an `enum` containing some of the courses that a GISTRE must attend, such
as MAUTO, RGW (us :D), BSP...

#### Fields

- `motivation: i32` Motivation can be negative for a GISTRE...
- `happy: bool` A GISTRE is happy during the Rust workshop!

#### Methods

- `attend_class(self, enum class)`

## 2 - Encapsulating everything

Right now everything is public, which is a bit sad when it comes to object oriented. Let's
make constructors and private functions.

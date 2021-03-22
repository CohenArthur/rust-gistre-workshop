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
make constructors and private functions. Let's also add personel, whose job is to take
care of the animals and make sure they're happy.

### Encapsulation

Just like every other object oriented programming languages, keeping fields public is
consideree bad practice. One of the reasons for this is that, if you decide to change the
inner workings of your libraray, your users would also be affected and need to change their
programs. If you, however, choose to keep to a specific public API, you are free to change
the fields of objects as you want. Let's remove all the public fields from our accessors.

You will notice a few things: First of all, we cannot instanciate the animals as we used
to do (`Ape { banana_counter: 165, ... }`) since the fields are now private. Secondly, you
cannot access those fields anymore, so your wranglers will have a bit of trouble if they
can't know how many bananas an ape still has.

When it comes to instanciation, there are no "constructors" in Rust. Instead, there are
static functions whose purpose is to return a filled instance of the object.
Let's create the following "constructors":

- `Ape::with_bananas(bananas: u32)`
- `Ape::new()`
- `Gistre::sad()`
- `Gistre::happy()`

### Personel

#### ApeWrangler

An Ape wrangler can take care of a few apes at once. Their job is to refill their bananas,
and keep them happy.

Let's create our empty `ApeWrangler` struct so we can add methods to it.

Your ApeWrangler should expose only one function through its public API. However, the type
itself should have multiple functions in order to separate your code correctly. For example,
the public function could be called `ApeWrangler::take_care()` and it would have to
refill the banana reserves and make the monkeys happy. Let's implement the following
function:

- `ApeWrangler::take_care(apes: &mut [&mut Ape])`

The `Ape` type must stay encapsulated! You might need to add other accessors to the struct.

You can add more complexity to this wrangler, for example by giving them an original
banana reserve that they have to divide equally between all apes based on their needs,
etc etc etc...

## 3 - Going further - Adding traits

I'm sure you've noticed that the role of an ApeWrangler is very simple. So simple, in fact
that it could be generalized to different monkeys. After all, you only need to replenish
their banana reserve and play with them a bit to make them happy.

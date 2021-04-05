# Let's implement capitalism

## 1 - A simple bank

Using the following type alias, create a `Bank` type that can hold any amount of money.
Said banks should be able to be displayed on stdout, and show statistics about themselves.

```rust
type Money = u64;
```

Because this is capitalism and everyone is rich, we assume that we cannot have negative
money. On the contrary, we can have up to 18446744073709551615 monies, Jeff Bezos style.

A bank should be a collection of money, for exemple by keeping a list of accounts associated
with someone's name. You should add methods to your bank in order to ease up adding up
a new account or adding money to an account (we can't lose money under capitalism, obviously).

Here's a non-exhaustive list of methods you could add:

- `add_account(account_name)`
- `add_money(account_name, money_amount)`

## 2 - Let's make our banks generic

Banks usually do not deal with `u64`s, but rather with dollars, euros, or yens. Don't you
think that our banks should reflect that?

Create a `Money` trait in order to represent what a currency is. Then, implement that trait
for a few money types, such as `Dollar`, `Euro` and `Ouguiya`, the malaysian currency.

Considering our moneys all need a way to compare themselves, we need to at least have a
`value` method. This method should return a `f64` and represent the dollar value of our
currency. Therefore, calling `value` on 89 dollars will return `89.0`. Calling `value` on
89 euros will return `89 * exchange_rate` dollars. (You could also add a few other methods
in order to get the exchange rate to dollars for that money for example)

The `Ouguiya` currency is interesting as it does not have "cents": It is a non-decimal
currency. Therefore, you cannot have 16.2 Ouguiya. Only 16 or 17.

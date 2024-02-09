---
theme: notty.json
author: ""
date:  ""
paging: 
---

# Doing Magic

---

# Case Study

Let's take a simple example:

<br>

> You wish to implement a type


> This type should internally do `-1` on the data


> The data will be `unsigned integer`


> What's the problem?

---

## Problems

<br>

> _If the data is `0` then, 💥!_

---

## How can we solve this?

---

> We can encode this information in the type system (probably)

> There are 2 ways to do this:

---

## Scenario 1

<br>

> We can have a runtime check, constructing the type only if the number is greater that `0`.

> Is this any difficult to do?

---

> No!

---

> Is this already done?

---

> Yes!

<br>

`std::num::NonZeroUsize`

---

Am I going to talk about it?

---

> No!

---

## Scenario 2

> Let's say we don't even want developers to manually code in a non-zero value, 
> (i.e. rejecting to compile if the value is `0`)

<br>

> Can we do this?

---

Let's look at a quick code example!

---

Let's checkout `magic`!

---

What are **dependent types**?

---

> a dependent type is a type whose definition depends on a value

---

Simply put, 

- The developer can define the behaviour of the type for different values of the type

---

A fun fact! 🙂 

---

Everything that I told you is a lie!

---

Rust doesn't have dependent types

---

So, how did I just do it?

--- 

The Answer: I didn't

---

Thank you


[The pi type trilogy](https://github.com/rust-lang/rfcs/issues/1930)

[Dependent Types](https://www.youtube.com/watch?v=BdXWlQsd7RI)

# Variables in Rust

Rust is statically and strongly typed language , It means that the complier should know all the types of all the variables at the complie time.
Sometimes the complier can infer the type ( strings with quotes ) sometimes we need to declare the types

1. RUST by default when we declare a variable with the let keyword , Its immutable
2. In RUST we can define a variable multiple times in the same scope , which means we can redefine the variable with the same name in the same scope . We can use the previous defined value even during assigning the new variable with the same name
3. If the have a mutable variable , it means that the value of the variable can change but not the type that means we cannot change the value of the varibale from an integer to a string and etc , unless I use shadowing.

## Constants

### Why do we need constants if we already have let which is immutable ?

1. We need to provide the type of the variable for the const values , the complier will let us know what type to use according to the value used there
2. The const variables have to be declared and assign the value in the same time

### Shadowing works with constant or not ?

It will not work

### will mut keyword work with constants or not ?

const cannot be mutable , it can be decleared as static

## Data Types in RUST

There are two major data types in RUST

1. Scalar : A scalar type represents a single value.
2. Compound :

## Scalar Type in RUST

Rust has four primary scalar types:

1. integers : There are two types **Signed and Unsigned**
2. floating-point numbers,
3. Booleans, and
4. characters.

## Integer

1. integer types default to i32.
2. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

## Floating Point

To store decimal values
There are only 2 types of floating points

1. f32
2. f64

we cant do this because we cant take in the integer value and subtract from the floating point value
When doing the math operations , both the variables have to in the same data type

### Lets compare the difference betwene the integer which is signed and unsigned

In base 2 numeric system , the number can be either one or zero and the first bit of the number denotes the sign of the number

- signed integer = Both positive and negative values , _range of the signed integer is limited because of the sign bit in the begining_
- unsigned integer = can have only positive value , which also means that the _range of the unsigned integer is more_ because there is no resever for the sign of the number

### Type Casting : Converting one data type to another data type

1. Coverting one data type to another data type

2. Unit Type : Bascially it represents the singular value which is completely and it is represented with empty parantheses , this is kinda like the database field where we can have null or void or even can we have empty string value.

   - python = Nan value
   - other languages = empty array

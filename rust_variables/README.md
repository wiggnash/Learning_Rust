# Variables in Rust

Rust is statically and strongly typed language , It means that the complier should know all the types of all the variables at the compplie time.
Sometimes the complier can infer the type ( strings with quotes ) sometimes we need to declare the types

1. RUST by default when we declare a variable with the let keyword , Its immutable
2. In RUST we can define a variable multiple times in the same scope , which means we can redefine the variable with the same name in the same scope . We can use the previous defined value even during assigning the new variable with the same name
3. If the have a mutable variable , it means that the value of the variable can change but not the type that means we cannot change the value of the varibale from an integer to a string and etc , unless I use shadowing.

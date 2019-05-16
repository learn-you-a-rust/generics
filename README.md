# generics
https://doc.rust-lang.org/stable/book/ch10-01-syntax.html

You can combine traits with generic types to constrain a generic type to only those types that have a particular behavior, as 
opposed to just any type.

Lifetimes are a variety of generics that give the compiler information about how references relate to each other.

This (a vector) is a slice:
`let number_list = vec![34, 50, 25, 100, 65];`

`<T>` is called a type declaration, ex. used in a function signature.

`fn largest<T>(list: &[T]) -> T {`
We read this definition as: the function largest is generic over some type `T`. This function has one parameter named `list`, which
is a slice of values of type `T`. The largest function will return a value of the same type `T`.

`std::cmp::PartialOrd` is an example of a trait.

Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures.

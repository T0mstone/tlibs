# BiResult
This library provides a single  type, `BiResult`.

A function that returns a `BiResult` always returns a usable result value, as well as any number of errors.

This is very useful for tasks like parsing, where you always want some usable result, whether there are errors or not.  
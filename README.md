# Homonym

Work completed on a language that allows for variable overloading, but no function or operator overloading.

Once it's completed, the language should look something like this:

```
>>> x the int = 3;
>>> x the float = 5.5;
>>> x +<int, int> x;
6
>>> x +<float, int> x;
8.5
>>> double_float(x);
11.0
>>> double_int(x);
6
```

While I don't think this would be a language I would ever like to program in, I think it's an interesting project to undertake.

Currently, I'm working on writing the parser and typechecker

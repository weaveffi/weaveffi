Build and run the C example

1. Ensure `libcalculator.dylib` is built in `target/debug`.
2. Build the example:

```
cc -I ../../generated/c main.c -L ../../target/debug -lcalculator -o c_example
```

3. Run:

```
DYLD_LIBRARY_PATH=../../target/debug ./c_example
```

First post cc project at 42

# Doc

- [Stanford-Guide to x86-64](https://web.stanford.edu/class/cs107/guide/x86-64.html)
- [Stanford-CS107 x86-64 Reference Sheet](https://web.stanford.edu/class/archive/cs/cs107/cs107.1252/resources/x86-64-reference.pdf)
- [Notes on x86-64 programming](https://usr.lmf.cnrs.fr/~jcf/ens/compil/x86-64.pdf)
- [Reverse Engineering For Everyone](https://0xinfection.github.io/reversing/reversing-for-everyone.pdf)
- [System V Application Binary Interface](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf)
- [System V Calling convention](https://en.wikipedia.org/wiki/X86_calling_conventions#x86-64_calling_conventions)
- [Conditionnal jump instructions](https://www.philadelphia.edu.jo/academics/qhamarsheh/uploads/Lecture%2018%20Conditional%20Jumps%20Instructions.pdf)

# Building the project

## Build the lib

```
make // will build the libasm.a binary
```
```
make test // will build the test binary and cp the executable in the project root
```
```
make clean // will delete the .build dir in the root of the repo
```
```
make fclean // will delete the libasm.a bin
```
```
make tclean // will delete the test exec, and run cargo clean in the test dir
```
```
make gclean // will run fclean and tclean
```
```
make gen // will compile the lib and the tests
```
```
make gre
```
```
make tre
```

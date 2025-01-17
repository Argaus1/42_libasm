# Doc

- [Stanford-Guide to x86-64](https://web.stanford.edu/class/cs107/guide/x86-64.html)
- [Stanford-CS107 x86-64 Reference Sheet](https://web.stanford.edu/class/archive/cs/cs107/cs107.1252/resources/x86-64-reference.pdf)
- [Notes on x86-64 programming](https://usr.lmf.cnrs.fr/~jcf/ens/compil/x86-64.pdf)
- [Reverse Engineering For Everyone](https://0xinfection.github.io/reversing/reversing-for-everyone.pdf)
- [System V Application Binary Interface](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf)

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

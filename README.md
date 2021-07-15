# shellcode-rs
`shellcode-rs` is a shellcode runner for Windows written it Rust and assembly. It may be useful for testing shellcode.

## Usage

If you have shellcode in a file `test.bin`, you can run it:

```
shellcode-rs.exe test.bin
```

The runner can use multiple execution methods to test different scenarios:

* `tail` - **Tail call** uses a `jmp` at the end of the calling function
* `ret` - **Ret call** overwrites the return address with the address of the shellcode and then returns into it

You can try out these methods with the `-m` flag like so:

```
shellcode-rs.exe -m tail test.bin
```

# Jack Compiler
This is a compiler front-end for the Jack programming language. The Jack programming language is a simple object-based language, specified in the amazing book [The Elements of Computing Systems](https://www.nand2tetris.org/book).

Here is a Hello World program in Jack:
```
class Main {
   function void main() {
      do Output.printString("Hello world!");
      do Output.println();
      return;
   }
}
```

The compiler front-end compiles Jack code into VM code that is then executed on a virtual machine.


## Usage
Compile the Jack compiler:
```
cargo build --release
```
Compile a Jack file:
```
jack_compiler <INPUT_FILE>.jack
```
Compile a directory of Jack files:
```
jack_compiler <INPUT_DIR>
```

## Documentation
To read the documentation, open the file [doc/jack_compiler/index.html](doc/jack_compiler/index.html) in a browser.


## Tests
The tests compare the outputs of each component (parser, tokenizer, VM writer) to the outputs of the reference implementation.

Running all tests:
```
cargo test
```

## License
[MIT](https://mit-license.org/)


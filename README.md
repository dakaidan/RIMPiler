# RIMPiler

A compiler for the rimp programming language.

## Documents

- [BSPR](./Documentation/BSPR/BSPR.pdf)
- [Report](./Documentation/Report/Report.pdf)

## Building

To build the compiler, you must have cargo installed.
    
```bash
cargo build --release
```

This will produce the compiler executable in `target/release/RIMPiler`.

The Krakatau assembler is required to assemble the output of the compiler. 
It is bundled with the project, however, if this does not work on certain systems,
you can build it separately by running the following commands:

```bash
cd external_dependencies/krakatau
cargo build --release
```

And either copy the executable to the same directory as the compiler:

```bash
cp target/release/krak2 ../../target/release/
```

Or add the path to the executable to your PATH environment variable.

## Running

To run the compiler, you can use the following command:

```bash
./RIMPiler -h
```

This will show you the help message for the compiler.

If you want to evaluate a program, you can use the following command:

```bash
./RIMPiler -r -i <program>
```

This will evaluate the program and print the result to the console.

If you want to run the abstract machine on a program, you can use the following command:

```bash
./RIMPiler -m -i <program>
```

This will run the abstract machine on the program and print the final state of the machine.

If you want to compile a program to Java class files, you can use the following command:

```bash
./RIMPiler -c -i <program> -o <output_folder>
```

This will compile the program to Java class files and save them in the output folder.
In order to run the compiled programs you must have Java installed.
On newer versions of Java (7+) you are likely to need to use the `-noverify` flag to run the programs due to missing stackmaps.

So when running the compiled programs you can use the following command:

```bash
java -noverify -cp <output_folder> Main
```

You may get a warning about this option being depreciated, however, this is currently needed to run the compiled programs.

You can always pass the `-h` flag to RIMPiler to see the help message.

## Abstract Machine

Within the abstract machine there are many commands available to the user.
These commands are as follows:
- `step` - Steps through the program one instruction at a time.
- `back` - Steps back through the program one instruction at a time.
- `print control stack` - Prints the control stack.
- `print back stack` - Prints the back stack.
- `print result stack` - Prints the result stack.
- `print store` - Prints the store.
- `print all` - Prints all of the above.
- `step and print` - Steps through the program one instruction at a time and prints the control stack, back stack, result stack and store.
- `run` - Runs the program until it terminates.
- `run and print` - Runs the program until it terminates and prints the control stack, back stack, result stack and store at each step.
- `run print reverse` - Runs the program until it terminates and prints the control stack, back stack, result stack and store at each step then repeats in reverse.
- `run until rule` - Runs the program until the rule is reached.
- `direction` - Displays the current direction of the machine.
- `reverse` - Reverses the direction of the machine.
- `rule` - Displays the current rule to be applied.
- `rules` - Displays the rules that can be applied.
- `quit` - Quits the abstract machine.
- `help` - Displays the help message.

Each of these have a shortened version that can be used as well, these can be found by typing `help` into the abstract machine.
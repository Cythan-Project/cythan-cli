# cythan-cli
 A CLI that uses cythan-compiler and cythan to build, test and execute Cythan V3 Code

## Usage

### Compile
This will compile the file `cythan.ct` to `compiled.cct`
```
 $ cythan-cli compile cythan.ct
```

You can define the output file:
```
 $ cythan-cli compile cythan.ct compiled-custom.cct
```

If you want raw (readable by human but way larger) compiled cythan use the `.rct` extension for your output path:
```
 $ cythan-cli compile cythan.ct compiled-custom.ct
```

The compiler can also compress `.rct` files.

### Execute
This will execute both raw compiled file (`.rct`) or compressed compiled ones (`.cct`):
```
 $ cythan-cli execute cythan.cct 100
```
In this case 100 iterations of Cythan are computed and drawn to the terminal.

If you want the output be be written to a file: (You can use both `.rct` and `.cct`)
```
 $ cythan-cli execute cythan.cct 100 executed.rct
```

### Decompile
This will convert `.cct` file into `.rct` ones.

In this example `cythan.cct` will be decompiled to `decompiled.rct`
```
 $ cythan-cli decompile cythan.cct decompiled.rct
```
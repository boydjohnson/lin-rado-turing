# A tool for running Turing Machines

```
turing 
Turing Machine VM

USAGE:
    turing [FLAGS] [OPTIONS] <program> [output]

FLAGS:
    -h, --help        Prints help information
    -p, --parallel    Run the recurrence check in parallel
    -V, --version     Prints version information
    -v, --verbose     Log each step's state and symbol.

OPTIONS:
        --blank <blank>               Check blanking beaver starting at this step
    -c, --check <check-recurrence>    Run the recurrence check, taking more time
        --limit <limit>               Number of steps to limit the VM to.

ARGS:
    <program>    The Turing program. eg 1RB 0LA 1RB 0LH
    <output>     Filename to write output to or - for stdout.
```

## Program string scheme
There are 2 spaces between the instructions for each state and 1 space between each instruction for each symbol.

```bash
2-state 2-symbol: 1RB 0LA  0RA 1LB
```

This tool can handle 2-state up through 6-states and 2-symbols through 4-symbols.
`A-F + H` for states and `0-3` for symbols.

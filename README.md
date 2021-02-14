# A tool for running Turing Machines

```
turing -h
turing 
Turing Machine VM

USAGE:
    turing [FLAGS] [OPTIONS] <complexity> <program> [output]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Log each step's state and symbol.

OPTIONS:
    -c, --check <check-recurrence>    Run the recurrence check, taking more time
        --limit <limit>               Number of steps to limit the VM to.

ARGS:
    <complexity>    The number of states and number of symbols. eg 3-2, 4-2, 2-4...
    <program>       The Turing program. eg 1RB 0LA 1RB 0LH
    <output>        Filename to write output to or - for stdout.
```

## Can be used with `xargs` to compute in parallel.

```
cat file.txt | xargs -n 1 -P 4 -I{} turing --check 50000 --limit 100000 4-2 "{}" "{}".log
```
This produces a file with a result for each Turing Machine. Then you can `cat *.log` to see
all the results.

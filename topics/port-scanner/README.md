# Network Port Scanner

## Description and Goal

Build a *multi-threaded* command-line application that scans a range of ports at a URL or IP to check if they are open.

The tool displays all open ports at the target URL.

```shell
Usage: scanner [OPTIONS] URL/IP

Options:
 -p, --ports <PORT_RANGE> TCP or UDP port ranges. Can be set multiple times.
 -t, --threads <THREADS>  Max number of threads.
 -h, --help               Print help (see more with '--help')
 -V, --version            Print version
```

## References

[nmap](https://nmap.org/)

## Grade Factor

The grade factor for this project is *0.9*.

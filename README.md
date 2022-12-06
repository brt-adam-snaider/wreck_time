Tool for continuously modifying the wallclock time in the system.

# Usage

Install rust toolchain - https://rustup.rs

Once installed, use `cargo build --release`. The executable is in the `target/release/` directory.

Note: Modifying the wallclock time requires root privileges.

# Examples


Get help menu

```$ ./wreck_time --help```

Move wallclock forward once by 1s.

```$ sudo ./wreck_time oneshot -o 1s```

Move the wallclock forward at a frequency of 1Hz (every second). The increment will initially be 0s
and will go up to 10s by increments of 1s.

```$ sudo ./wreck_time linear --frequency 1 --start 0s --end 10s --increment 1s```

Move the wallclock forward at a frequncy of 2Hz (twice per second). The increment will be uniformly
chosen every iteration to be within 10ns and 1ms.

```$ sudo ./wreck_time random --frequency 2 --low 10ns --high 1ms```


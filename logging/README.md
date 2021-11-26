## Logging

Demonstrates multiple logging features/tricks for solana programs.

### Logging Rust Location

_This is most likely quite expensive, so use with care and mainly when still testing_.

```rs
use core::panic;

...

msg!("at {:?}", panic::Location::caller());

...
```

Prints something similar to the below when triggered via `cargo test-bpf` tests:

```
    [2021-11-26T18:38:35.052666000Z INFO  solana_program_test] "sol_logging" BPF program from /Volumes/d/dev/mp/solana/projects/solarium/target/deploy/sol_logging.so, modified 31 minutes, 3 seconds, 712 ms, 40 µs and 303 ns ago
    [2021-11-26T18:38:35.436936000Z DEBUG solana_runtime::message_processor] Program 4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM invoke [1]
    [2021-11-26T18:38:35.437205000Z DEBUG solana_runtime::message_processor] Program log: Instruction: LogRustLocation
--> [2021-11-26T18:38:35.439442000Z DEBUG solana_runtime::message_processor] Program log: at Location { file: "program/src/processor.rs", line: 71, col: 21 }
    [2021-11-26T18:38:35.440201000Z DEBUG solana_runtime::message_processor] Program log: LogSetup {
            is_initialized: false,
            times_invoked: 0,
        }
    [2021-11-26T18:38:35.440311000Z DEBUG solana_rbpf::vm] BPF instructions executed (interp): 12509
    [2021-11-26T18:38:35.440323000Z DEBUG solana_rbpf::vm] Max frame depth reached: 16
    [2021-11-26T18:38:35.440365000Z DEBUG solana_runtime::message_processor] Program 4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM consumed 12665 of 200000 compute units
    [2021-11-26T18:38:35.440441000Z DEBUG solana_runtime::message_processor] Program 4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM success
    test log_rust_location::success_log_rust_location ... ok
```

and the below when run inside a local validator with logs watched via `solana logs`:

```
Streaming transaction logs. Confirmed commitment
Transaction executed in slot 8:
  Signature: 5Q9MkGmW2Y9PtcnVAkyukP1WC9N7pP8A95gF1aqxKb1WhwVbVEoQccTEE634Z6WLRaqu2QQ3GZ1qvRQqgkDSDo2k
  Status: Ok
  Log Messages:
    Program 11111111111111111111111111111111 invoke [1]
    Program 11111111111111111111111111111111 success
Transaction executed in slot 9:
    
    ...
    
    Program log: Updating account data
    Program BZjfeRpCFk3cNjfJz9rJdk4o3uLZyvGfD4tizaf8WG7L consumed 3939 of 200000 compute units
    Program BZjfeRpCFk3cNjfJz9rJdk4o3uLZyvGfD4tizaf8WG7L success
    Program BZjfeRpCFk3cNjfJz9rJdk4o3uLZyvGfD4tizaf8WG7L invoke [1]
    Program log: Instruction: LogRustLocation
--> Program log: at Location { file: "program/src/processor.rs", line: 71, col: 21 }
    Program log: LogSetup {
    is_initialized: true,
    times_invoked: 0,
}
    Program BZjfeRpCFk3cNjfJz9rJdk4o3uLZyvGfD4tizaf8WG7L consumed 12652 of 200000 compute units
    Program BZjfeRpCFk3cNjfJz9rJdk4o3uLZyvGfD4tizaf8WG7L success
```

### I wanna see

Sure, just run `cargo test-bpf` or

from `./logging` run `./test/init-validator` and in another terminal `esr
test/log-rust-location.ts`.

## Dependencies

Should install with `yarn` except you may want to globally install
[`esbuild-runner`](https://github.com/folke/esbuild-runner) to get that `esr` command.

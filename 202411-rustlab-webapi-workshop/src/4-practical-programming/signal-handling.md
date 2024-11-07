<section class="slide">

# Signal handling

</section>
<section class="slide">

## SIGINT

Use the `ctrlc` crate:

```rust
use std::{thread, time::Duration};

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Unable to set Ctrl-C handler");


    thread::sleep(Duration::from_secs(2));
}
```

</section>
<section class="slide">

## Other signals

The `signal_hook` error has th best support:

```rust
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new([SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    println!("Press Ctrl+C to exit");
    thread::sleep(Duration::from_secs(100));
    Ok(())
}

```

</section>


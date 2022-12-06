use clap::Parser;
use rand::prelude::Distribution;
use std::time::Duration;

fn update_time(delta: Duration) {
    use core::mem::MaybeUninit;
    use libc::clock_gettime;
    use libc::clock_settime;
    use libc::timespec;

    unsafe {
        let mut time: MaybeUninit<timespec> = MaybeUninit::uninit();
        assert_eq!(clock_gettime(libc::CLOCK_REALTIME, time.as_mut_ptr()), 0);

        let time = time.assume_init();
        let time = Duration::new(time.tv_sec as u64, time.tv_nsec as u32);
        let time = time + delta;

        let time = timespec {
            tv_nsec: time.subsec_nanos() as i64,
            tv_sec: time.as_secs() as i64,
        };

        assert_eq!(
            clock_settime(libc::CLOCK_REALTIME, &time as *const timespec),
            0
        );
    }
}

/// Simple program to change the system clock.
#[derive(Parser, Debug)]
enum Args {
    /// Single change to the clock.
    Oneshot {
        #[arg(long, short)]
        offset: humantime::Duration,
    },

    /// Randomly update the clock using a uniform distribution where the delta is [low, high)
    Random {
        /// Frequency in Hz that defines how often to trigger a clock change.
        #[arg(long, short)]
        frequency: f64,
        /// Low bound for the clock change offset.
        #[arg(long, short)]
        low: humantime::Duration,
        /// High bound for clock change offset.
        #[arg(long, short)]
        high: humantime::Duration,
    },

    /// Sequentailly change the time starting from an offset, and increasing it until the end
    /// offset.
    Linear {
        /// Initial value for clock offset.
        #[arg(long, short)]
        start: humantime::Duration,
        /// Final value for clock offset.
        #[arg(long, short)]
        end: humantime::Duration,
        /// How much to increment our clock offset by on every iteration.
        #[arg(long, short)]
        increment: humantime::Duration,
        /// Frequency in Hz that defines how often to trigger a clock change.
        #[arg(long, short)]
        frequency: f64,
    },
}

fn main() {
    let args = Args::parse();

    match args {
        Args::Oneshot { offset } => {
            println!("Performing oneshot clock change by {offset}");
            update_time(offset.into())
        }
        Args::Linear {
            start,
            end,
            increment,
            frequency,
        } => {
            println!("Performing linear clock change");

            let mut delta = start.into();
            let increment = increment.into();
            let end = end.into();
            let sleep = Duration::from_secs_f64(1.0 / frequency);
            loop {
                println!("Updating time by {delta:?}");
                update_time(delta);
                std::thread::sleep(sleep);
                delta = std::cmp::min(delta + increment, end);
            }
        }
        Args::Random {
            frequency,
            low,
            high,
        } => {
            println!("Performing random clock change");
            let low: Duration = low.into();
            let high: Duration = high.into();
            let distribution = rand::distributions::Uniform::from(low..high);
            let sleep = Duration::from_secs_f64(1.0 / frequency);
            let mut rng = rand::thread_rng();
            loop {
                let delta = distribution.sample(&mut rng);
                println!("Updating time by {delta:?}");
                update_time(delta);
                std::thread::sleep(sleep);
            }
        }
    }
}

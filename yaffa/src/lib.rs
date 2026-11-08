//! Yet Another Fromework For Applications

pub use color_eyre::eyre::{Error, Result, eyre};

/// A minimalist application wrapper providing `color_eyre` errors and basic `clap` parsing support
pub trait App: Sized + RunWith<Self::Options> {
    /// The top-level [clap::Parser] options for this application
    type Options: clap::Parser;

    /// Run the main process with the default app state
    fn run_main_default() -> Result<()>
    where
        Self: Default,
    {
        Self::default().run_main()
    }

    /// Run the main process with the current app state
    fn run_main(self) -> Result<()> {
        use clap::Parser as _;

        color_eyre::install()?;

        let opts = Self::Options::parse();
        self.run_with(opts)
    }
}

/// An abstraction for running with an argument
pub trait RunWith<A> {
    /// Run with the given `arg`
    fn run_with(self, arg: A) -> Result<()>;
}

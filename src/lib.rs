//! # Sleep Report
//!
//! This application generates a sleep report from an iCalendar file.
//!
//! ## Architecture
//!
//! The application is divided into several modules:
//!
//! - `app`: Contains the core application logic, including the `App` struct and the `AppDependency` trait.
//! - `io`: Contains the input/output logic, including the iCalendar fetcher, sleep loader, and command-line options parser.
//! - `root`: Contains the application's startup logic.
//! - `sleep`: Contains the sleep data structures and algorithms.
//! - `util`: Contains utility functions.
//!
//! The application follows a dependency injection pattern. The `App` struct is generic over the `AppDependency` trait, which provides the application with its dependencies. This allows for easy testing and mocking of the application's dependencies.
//!
//! The application's main function is in `src/bin/sleepreport.rs`. This function initializes the application's dependencies and runs the application.
//!
//! ## Execution Phases
//!
//! The application runs in several distinct phases:
//!
//! 1.  **Parsing:** The command-line arguments are parsed into the `Opts` struct in the `io` module.
//! 2.  **Configuration:** The `root` module takes the `Opts` and creates a fully configured `AppConfig` and a `Dependencies` struct. This is where default values are applied and the report's date range is calculated.
//! 3.  **Execution:** The `App` struct is created with the `Dependencies` and its `run` method is called. This orchestrates the following steps:
//!     a.  **Fetch:** The `IcalFetcher` reads the raw iCalendar file.
//!     b.  **Load:** The `SleepLoader` parses the iCalendar data into a series of sleep `Range`s.
//!     c.  **Count:** The `SleepCounter` processes the sleep `Range`s into a `SleepTable`.
//!     d.  **Report:** The `SleepReporter` takes the `SleepTable` and generates the final, formatted string report, which is then printed to the console.

pub mod app;
pub mod io;
pub mod root;
pub mod sleep;
pub mod util;
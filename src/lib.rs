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

pub mod app;
pub mod io;
pub mod root;
pub mod sleep;
pub mod util;

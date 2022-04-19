mod standard;

pub use self::standard::GuessingNumber;

#[cfg(feature = "wolfram")]
mod wolfram;

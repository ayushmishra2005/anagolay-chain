/// Money matters.
pub mod currency {
  use crate::Balance;

  pub const UNITS: Balance = 1_000_000_000_000;
  pub const DOLLARS: Balance = UNITS; // 1_000_000_000_000
  pub const CENTS: Balance = DOLLARS / 100; // 10_000_000_000
  pub const MILLICENTS: Balance = CENTS / 1_000; // 10_000_000

  pub const TRANSACTION_BYTE_FEE: Balance = 1 * UNITS;
  pub const STORAGE_BYTE_FEE: Balance = 100 * UNITS;
  pub const WEIGHT_FEE: Balance = 50 * UNITS;

  pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
  }
}

/// Time and blocks.
pub mod time {
  use crate::{BlockNumber, Moment};

  /// This determines the average expected block time that we are targeting.
  /// Change this to adjust the block time.
  pub const MILLISECS_PER_BLOCK: Moment = 6000;

  // NOTE: Currently it is not possible to change the slot duration after the chain has started.
  // Attempting to do so will brick block production.
  pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

  // These time units are defined in number of blocks.
  pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
  pub const HOURS: BlockNumber = MINUTES * 60;
  pub const DAYS: BlockNumber = HOURS * 24;
  pub const WEEKS: BlockNumber = DAYS * 7;
}

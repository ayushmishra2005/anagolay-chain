/// Money matters.
pub mod currency {
  use crate::Balance;

  // Provide a scaling factor
  pub const SUPPLY_FACTOR: Balance = 10;

  pub const UNITS: Balance = 1_000_000_000_000;
  pub const THOUSANDTHS: Balance = UNITS / 1_000; // 1_000_000_000
  pub const MILLIONTHS: Balance = UNITS / 1_000_000; // 1_000_000
  pub const BILLIONTHS: Balance = UNITS / 1_000_000_000; // 1_000
  pub const TRILLIONTHS: Balance = UNITS / 1_000_000_000_000; // 1

  pub const TRANSACTION_BYTE_FEE: Balance = 1 * TRILLIONTHS * SUPPLY_FACTOR;
  pub const STORAGE_BYTE_FEE: Balance = 100 * TRILLIONTHS * SUPPLY_FACTOR;
  pub const WEIGHT_FEE: Balance = 50 * TRILLIONTHS * SUPPLY_FACTOR;

  pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * TRILLIONTHS * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
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

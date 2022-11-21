/// Money matters.
pub mod currency {
  use crate::Balance;

  // Provide a scaling factor
  pub const SUPPLY_FACTOR: Balance = 10;

  pub const UNITS: Balance = 1_000_000_000_000;
  pub const MILLI: Balance = UNITS / 1_000; // 1_000_000_000
  pub const MICRO: Balance = UNITS / 1_000_000; // 1_000_000
  pub const NANO: Balance = UNITS / 1_000_000_000; // 1_000
  pub const PICO: Balance = UNITS / 1_000_000_000_000; // 1

  pub const TRANSACTION_BYTE_FEE: Balance = 1 * PICO * SUPPLY_FACTOR;
  pub const STORAGE_BYTE_FEE: Balance = 100 * PICO * SUPPLY_FACTOR;
  pub const WEIGHT_FEE: Balance = 50 * PICO * SUPPLY_FACTOR;

  pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * PICO * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
  }
}

/// Time and blocks.
pub mod time {
  use crate::{BlockNumber, Moment};

  /// This determines the average expected block time that we are targeting.
  /// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
  /// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
  /// up by `pallet_aura` to implement `fn slot_duration()`.
  ///
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

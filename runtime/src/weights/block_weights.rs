// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

pub mod constants {
  use frame_support::{
    parameter_types,
    weights::{constants, Weight},
  };

  parameter_types! {
    /// Importing a block with 0 Extrinsics.
    pub const BlockExecutionWeight: Weight = constants::WEIGHT_PER_NANOS.saturating_mul(5_000_000);
  }

  #[cfg(test)]
  mod test_weights {
    use frame_support::weights::constants;

    /// Checks that the weight exists and is sane.
    // NOTE: If this test fails but you are sure that the generated values are fine,
    // you can delete it.
    #[test]
    fn sane() {
      let w = super::constants::BlockExecutionWeight::get();

      // At least 100 µs.
      assert!(
        w.ref_time() >= 100u64 * constants::WEIGHT_PER_MICROS.ref_time(),
        "Weight should be at least 100 µs."
      );
      // At most 50 ms.
      assert!(
        w.ref_time() <= 50u64 * constants::WEIGHT_PER_MILLIS.ref_time(),
        "Weight should be at most 50 ms."
      );
    }
  }
}

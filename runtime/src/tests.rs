use crate::{BlockWeights, MinimumMultiplier, Runtime, SlowAdjustingFeeUpdate, System, TargetBlockFullness};
use frame_support::pallet_prelude::*;
use sp_runtime::traits::Convert;

fn run_with_system_weight<F>(w: Weight, mut assertions: F)
where
  F: FnMut() -> (),
{
  let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
    .build_storage::<Runtime>()
    .unwrap()
    .into();
  t.execute_with(|| {
    System::set_block_consumed_resources(w, 0);
    assertions()
  });
}

#[test]
fn multiplier_can_grow_from_zero() {
  let minimum_multiplier = MinimumMultiplier::get();
  let target = TargetBlockFullness::get() * BlockWeights::get().get(DispatchClass::Normal).max_total.unwrap();
  // if the min is too small, then this will not change, and we are doomed forever.
  // the weight is 1/100th bigger than target.
  run_with_system_weight(target * 101 / 100, || {
    let next = SlowAdjustingFeeUpdate::<Runtime>::convert(minimum_multiplier);
    assert!(next > minimum_multiplier, "{:?} !>= {:?}", next, minimum_multiplier);
  })
}

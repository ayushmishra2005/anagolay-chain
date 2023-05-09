## Definition

Up-to-date docs are on https://anagolay.dev/docs/anagolay/tipping/

Tipping pallet is the core feature in our attempt to build the functionality and features to support creators’ economy in a truly decentralized manner. Every creator can verify their revenue channels like websites, subdomains, or a username on commercial websites and accept payment from anybody in crypto. This pallet, together with the Anagolay Extension, can be used to support open-source projects per user and per repository. To prevent the misuse of the pallet and to make sure that the correct people are supported, the tipping pallet depends on the verification pallet to get the proofs of the domain, username or repository verification, and statements pallet for the ownership.

**Configuration**

The runtime needs to configure the tipping pallet as follows:

```rust
  impl tipping::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type TimeProvider = pallet_timestamp::Pallet<Runtime>;
    type WeightInfo = tipping::weights::AnagolayWeight<Runtime>;

    // Limit on the maximum number of tips that will be recorded, per context
    const MAX_TIPS_PER_VERIFICATION_CONTEXT: u32 = 10000;
  }
```


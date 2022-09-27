/// Macro that generates a wrapper for a constant to satisfy `trait Get`.
/// The constant is expected to be configurable through the pallet
/// configuration
///
/// # Arguments
///   - const_name: Constant name
///   - const_type: Type of the constant value
#[macro_export]
macro_rules! getter_for_constant {
  ($const_name:ident, $const_type:ident) => {
    $crate::paste::paste! {
      /// Getter for a configurable constant. Refer to [`getter_for_constant`] macro for the details
      #[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, frame_support::sp_runtime::RuntimeDebug, frame_support::pallet_prelude::TypeInfo)]
      pub struct [<$const_name Get>] <T> (frame_support::pallet_prelude::PhantomData<T>);
      /// Implementation of the ['Get'] trait for the getter of a configurable constant
      impl<T: crate::pallet::Config> frame_support::pallet_prelude::Get<$const_type> for [<$const_name Get>] <T> {
        fn get() -> $const_type {
          T :: [<$const_name:snake:upper>]
        }
      }
    }
  };
}

/// Macro that generates a wrapper for a constant to satisfy `trait Get`.
/// The constant is hardcoded
///
/// # Arguments
///   - const_name: Constant name
///   - const_type: Type of the constant value
#[macro_export]
macro_rules! getter_for_hardcoded_constant {
  ($const_name:ident, $const_type:ident, $const_value:tt) => {
    $crate::paste::paste! {
      /// Hard-coded constant. Refer to [`getter_for_hardcoded_constant`] macro for the details
      pub const [<$const_name:snake:upper>] : $const_type = $const_value;
      /// Getter for an hard-coded constant
      #[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, frame_support::sp_runtime::RuntimeDebug, frame_support::pallet_prelude::TypeInfo)]
      pub struct [<$const_name Get>]();
      /// Implementation of the ['Get'] trait for the getter of an hard-coded constant
      impl frame_support::pallet_prelude::Get <$const_type> for [<$const_name Get>] {
        fn get() -> $const_type {
          [<$const_name:snake:upper>]
        }
      }
    }
  };
}

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
      /// Getter for ['[<$const_name:snake:upper>]`]
      #[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, frame_support::sp_runtime::RuntimeDebug, frame_support::pallet_prelude::TypeInfo)]
      pub struct [<$const_name Get>] <T> (frame_support::pallet_prelude::PhantomData<T>);
      /// Implementation of the ['Get'] trait for ['[<$const_name Get>]']
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
      /// Hard-coded $const_name of type $const_type with value $const_value
      const [<$const_name:snake:upper>] : $const_type = $const_value;
      /// Getter for ['[<$const_name:snake:upper>]`]
      #[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, frame_support::sp_runtime::RuntimeDebug, frame_support::pallet_prelude::TypeInfo)]
      pub struct [<$const_name Get>]();
      /// Implementation of the ['Get'] trait for ['[<$const_name Get>]']
      impl frame_support::pallet_prelude::Get <$const_type> for [<$const_name Get>] {
        fn get() -> $const_type {
          [<$const_name:snake:upper>]
        }
      }
    }
  };
}

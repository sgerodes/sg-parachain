#[cfg(feature = "std")]
use crate::sp_version::NativeVersion;
use crate::sp_version::RuntimeVersion;
pub use crate::apis::RUNTIME_API_VERSIONS;

#[polkadot_sdk::sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: alloc::borrow::Cow::Borrowed("parachain-runtime"),
    impl_name: alloc::borrow::Cow::Borrowed("parachain-runtime"),
    authoring_version: 1,
    spec_version: 1,
    impl_version: 0,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    system_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
#[allow(dead_code)]
pub fn native_version() -> NativeVersion {
    NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

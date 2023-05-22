// Unfortunately, path is relative to a temporary build directory :-/
uniffi_macros::generate_and_include_scaffolding!("../../../../fixtures/trait-methods/src/trait_methods.udl");

fn main() { /* empty main required by `trybuild` */}

// We derive `Debug` so our generated `assert_impl_all` output is prettier, but
// we don't derive any other traits used in that UDL.
#[derive(Debug)]
pub struct TraitMethods {}

impl TraitMethods {
    fn new(name: String) -> Self {
        unreachable!();
    }
}

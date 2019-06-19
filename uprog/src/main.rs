use lib123::VisibleToUsers;

fn main() {
    // this will fail if lib + internal is moved to a sep. crate
    // because ImplementorCrateOnly is going to be leaked
    let used = VisibleToUsers::from_args_for_specific_impl(5);
    
    used.use_internal();
}

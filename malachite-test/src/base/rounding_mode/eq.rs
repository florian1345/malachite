use common::{DemoBenchRegistry, NoSpecialGenerationMode};
use inputs::base::pairs_of_rounding_modes;

pub fn register(registry: &mut DemoBenchRegistry) {
    register_ns_demo!(registry, demo_rounding_mode_eq);
}

fn demo_rounding_mode_eq(gm: NoSpecialGenerationMode, limit: usize) {
    for (x, y) in pairs_of_rounding_modes(gm).take(limit) {
        if x == y {
            println!("{} = {}", x, y);
        } else {
            println!("{} ≠ {}", x, y);
        }
    }
}

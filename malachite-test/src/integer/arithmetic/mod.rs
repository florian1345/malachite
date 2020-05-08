use common::DemoBenchRegistry;

pub mod abs;
pub mod add;
pub mod add_mul;
pub mod div;
pub mod div_exact;
pub mod div_mod;
pub mod div_round;
pub mod divisible_by;
pub mod divisible_by_power_of_two;
pub mod eq_mod;
pub mod eq_mod_power_of_two;
pub mod mod_op;
pub mod mod_power_of_two;
pub mod mul;
pub mod neg;
pub mod parity;
pub mod power_of_two;
pub mod round_to_multiple_of_power_of_two;
pub mod shl_i;
pub mod shl_round_i;
pub mod shl_u;
pub mod shr_i;
pub mod shr_round_i;
pub mod shr_round_u;
pub mod shr_u;
pub mod sign;
pub mod sub;
pub mod sub_mul;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    abs::register(registry);
    add::register(registry);
    add_mul::register(registry);
    div::register(registry);
    div_exact::register(registry);
    div_mod::register(registry);
    div_round::register(registry);
    divisible_by::register(registry);
    divisible_by_power_of_two::register(registry);
    eq_mod::register(registry);
    eq_mod_power_of_two::register(registry);
    mod_op::register(registry);
    mod_power_of_two::register(registry);
    mul::register(registry);
    neg::register(registry);
    parity::register(registry);
    power_of_two::register(registry);
    round_to_multiple_of_power_of_two::register(registry);
    shl_i::register(registry);
    shl_round_i::register(registry);
    shl_u::register(registry);
    shr_i::register(registry);
    shr_round_i::register(registry);
    shr_round_u::register(registry);
    shr_u::register(registry);
    sign::register(registry);
    sub::register(registry);
    sub_mul::register(registry);
}

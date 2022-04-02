use malachite_base::test_util::hash::hash;
use malachite_nz::test_util::generators::natural_gen;

#[test]
fn hash_properties() {
    natural_gen().test_properties(|x| {
        assert_eq!(hash(&x), hash(&x.clone()));
    });
}

mod mask;
pub mod charray;


/// passes if the left is a subset of the right
#[macro_export]
macro_rules! assert_is_subset {
    ($left:expr, $right:expr) => {
        let mut missing = vec![];
        for m in $left {
            if !$right.contains(&m) {
                missing.push(m);
            }
        }

        if missing.len() > 0 {
            panic!("assertion failed, set difference: {:?}", missing);
        }
    };
}

/// This is essentially assert_eq but doesn't care about order differences
#[macro_export] macro_rules! assert_are_equal_sets {
    ($left:expr, $right:expr) => (
        assert_is_subset!(&$left, &$right);
        assert_is_subset!(&$right, &$left);
    );
}


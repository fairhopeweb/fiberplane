use crate::operations::{
    error::*,
    fixtures::{TEST_CASES, TEST_NOTEBOOK},
    invert_operation,
};
use pretty_assertions::assert_eq;

#[test]
pub fn test_invert_operation() -> Result<(), Error> {
    // All test cases start with the `TEST_NOTEBOOK` from the fixtures, after which the
    // operation from the test case is applied, then we apply the inverted operation
    // and make sure the result matches the original notebook.
    for test_case in TEST_CASES.iter() {
        assert_eq!(
            TEST_NOTEBOOK
                .apply_operation(&test_case.operation)?
                .apply_operation(&invert_operation(&test_case.operation))?,
            *TEST_NOTEBOOK,
            "Operation could not be cleanly reverted!\n\
                Operation: {:?}\n\
                Inverted: {:?}",
            &test_case.operation,
            invert_operation(&test_case.operation),
        );
    }

    Ok(())
}

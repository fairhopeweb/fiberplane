use super::fixtures::TEST_NOTEBOOK;
use super::test_cases::TEST_CASES;
use crate::operations::error::*;
use pretty_assertions::assert_eq;

#[test]
pub fn test_apply_operation() -> Result<(), Error> {
    // All test cases start with the `TEST_NOTEBOOK` from the fixtures, after which the
    // operation from the test case is applied, and we verify the resulting notebook matches
    // the expected result.
    for test_case in TEST_CASES.iter() {
        let op = format!("{:?}", &test_case.operation);
        assert_eq!(
            TEST_NOTEBOOK.apply_operation(&test_case.operation)?,
            test_case.expected_apply_operation_result,
            "Operation being tested: {}",
            op
        );
    }

    Ok(())
}

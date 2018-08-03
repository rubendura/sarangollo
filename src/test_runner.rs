use std::panic;

pub struct TestCase<I, E> {
    pub description: &'static str,
    pub input: I,
    pub expected: E,
}

pub fn run<I, E, F>(test_cases: &[TestCase<I, E>], test_func: F)
where
    E: Copy + panic::RefUnwindSafe,
    I: Copy + panic::RefUnwindSafe,
    F: Fn(I, E) + panic::RefUnwindSafe,
{
    let mut failed = false;
    for test_case in test_cases.iter() {
        let result = panic::catch_unwind(|| test_func(test_case.input, test_case.expected));
        if result.is_err() {
            failed = true;
            println!("FAILED: {}", test_case.description)
        }
    }
    assert!(!failed, "Some test cases failed!");
}

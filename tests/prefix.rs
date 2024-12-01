use std::time::Instant;

use textmate_scope_selector_peg;

#[derive(Clone, Debug)]
struct PrefixTestCase {
    selector: String,
    input: String,
    expected: Option<char>,
}

#[test]
fn test_bench_prefix() {
    let start = Instant::now();
    for _i in 1..=100000 {
        match textmate_scope_selector_peg::parse("L:text.html.markdown - (comment, string, meta.paragraph.markdown, markup.*.block.markdown)") {
            Ok(selector) => {
                (selector.get_prefix("text.html.markdown meta.paragraph.markdown"));
            },
            Err(_) => {},
        }
    }
    let elapsed = start.elapsed().as_nanos();
    println!("get_prefix - 100000 iters: {:.2?}", elapsed);
}

#[test]
fn test_prefix_match() {
    let test_cases = vec![
        PrefixTestCase {
            selector: "R:g".to_string(),
            input: "g".to_string(),
            expected: Some('R'),
        },
        PrefixTestCase {
            selector: "R:g".to_string(),
            input: "R:g".to_string(),
            expected: None
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.get_prefix(&case.input);
                assert_eq!(
                    result, case.expected,
                    "Test failed for selector: \"{}\", input: \"{}\"",
                    case.selector, case.input
                );
            }
            Err(err) => panic!("Parsing error for selector {}: {}", &case.selector, err),
        }
    }
}

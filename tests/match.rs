use std::time::Instant;

use textmate_scope_selector_peg;

#[derive(Clone, Debug)]
struct MatchTestCase {
    selector: String,
    input: String,
    expected: bool,
}

#[test]
fn test_bench_match() {
    let start = Instant::now();
    for _i in 1..=100000 {
        match textmate_scope_selector_peg::parse("source.matlab -comment -entity -support -string -variable -interpolation -source.shell") {
            Ok(selector) => {
                (selector.matches("source.matlab meta.class.matlab meta.class.declaration.matlab entity.name.type.class.matlab"));
            },
            Err(_) => {},
        }
    }
    let elapsed = start.elapsed().as_nanos();
    println!("matches - 100000 iters: {:.2?}", elapsed);
}

#[test]
fn test_atom_asterisk() {
    let test_cases = vec![
        MatchTestCase {
            selector: "*".to_string(),
            input: "a".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "*".to_string(),
            input: "b c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a.*.c".to_string(),
            input: "a.b.c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a.*.c".to_string(),
            input: "a.b.c.d".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a.*.c".to_string(),
            input: "a.b.d.c".to_string(),
            expected: false,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_atom_segment() {
    let test_cases = vec![
        MatchTestCase {
            selector: "a".to_string(),
            input: "a".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a".to_string(),
            input: "a.b".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a.b".to_string(),
            input: "a.b.c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a".to_string(),
            input: "abc".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a.b-c".to_string(),
            input: "a.b-c.d".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a.b".to_string(),
            input: "a.b-d".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "c++".to_string(),
            input: "c++".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "c++".to_string(),
            input: "c".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a_b_c".to_string(),
            input: "a_b_c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a_b_c".to_string(),
            input: "a_b".to_string(),
            expected: false,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_prefix_parse() {
    let test_cases = vec![
        MatchTestCase {
            selector: "L:g".to_string(),
            input: "g".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "R:g".to_string(),
            input: "R:g".to_string(),
            expected: false
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_composite_operator_disjunct() {
    let test_cases = vec![
        MatchTestCase {
            selector: "a | b".to_string(),
            input: "a".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a | b".to_string(),
            input: "b".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a | b".to_string(),
            input: "c".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a|b|c".to_string(),
            input: "c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a|b|c".to_string(),
            input: "d".to_string(),
            expected: false,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_composite_operator_negation() {
    let test_cases = vec![
        MatchTestCase {
            selector: "a - c".to_string(),
            input: "a b".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a - c".to_string(),
            input: "a".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "-c".to_string(),
            input: "b".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "-c".to_string(),
            input: "c b".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a-b".to_string(),
            input: "a b".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a -b".to_string(),
            input: "a b".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a -c".to_string(),
            input: "a b".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a-c".to_string(),
            input: "a b".to_string(),
            expected: false,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_composite_operator_conjunction() {
    let test_cases = vec![
        MatchTestCase {
            selector: "a & b".to_string(),
            input: "b a".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a&b&c".to_string(),
            input: "c".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a&b&c".to_string(),
            input: "a b d".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a & -b".to_string(),
            input: "a b d".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a & -b".to_string(),
            input: "a d".to_string(),
            expected: true,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_selector_comma() {
    let test_cases = vec![
        MatchTestCase {
            selector: "a,b,c".to_string(),
            input: "b c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a, b, c".to_string(),
            input: "d e".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a, b, c".to_string(),
            input: "d c.e".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a,".to_string(),
            input: "a c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a,".to_string(),
            input: "b c".to_string(),
            expected: false,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_group_brackets() {
    let test_cases = vec![
        MatchTestCase {
            selector: "(a,b) | (c, d)".to_string(),
            input: "a".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "(a,b) | (c, d)".to_string(),
            input: "b".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "(a,b) | (c, d)".to_string(),
            input: "c".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "(a,b) | (c, d)".to_string(),
            input: "d".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "(a,b) | (c, d)".to_string(),
            input: "e".to_string(),
            expected: false,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

#[test]
fn test_path_others_descendant() {
    let test_cases = vec![
        MatchTestCase {
            selector: "a b".to_string(),
            input: "a b".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a b".to_string(),
            input: "b a".to_string(),
            expected: false,
        },
        MatchTestCase {
            selector: "a c".to_string(),
            input: "a b c d e".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a b e".to_string(),
            input: "a b c d e".to_string(),
            expected: true,
        },
        MatchTestCase {
            selector: "a e".to_string(),
            input: "a b c d e".to_string(),
            expected: true,
        }
    ];

    for case in test_cases {
        match textmate_scope_selector_peg::parse(&case.selector) {
            Ok(selector) => {
                let result = selector.matches(&case.input);
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

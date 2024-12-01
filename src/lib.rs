extern crate peg;

pub use peg::{error::ParseError, str::LineCol};

pub mod matchers;

peg::parser! {
    pub grammar parser() for str {
        // Skip whitespace
        rule _() = quiet!{ [' ' | '\t']* }
        rule ws() = quiet!{ [' ' | '\t']+ }

        // Starting rule
        pub rule parse() -> Box<dyn matchers::Matcher>
            = _ selector:selector() _ {
                selector
            }

        // Atom matchers
        rule atom() -> Box<dyn matchers::Matcher>
            = segment:$(!['-']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '+']+) {
                Box::new(matchers::SegmentMatcher::new(segment))
            }
            / "*" {
                Box::new(matchers::TrueMatcher{})
            }

        // Scope matcher
        rule scope() -> Box<dyn matchers::Matcher>
            = atoms:atom() ++ "." {
                Box::new(matchers::ScopeMatcher::new(atoms))
            }

        // Path matcher
        rule path() -> Box<dyn matchers::Matcher>
            = prefix:$(['L'|'R'|'B'] ":")? scopes:scope() ++ ws() {
                Box::new(matchers::PathMatcher::new(prefix, scopes))
            }

        // Group matcher
        rule group() -> Box<dyn matchers::Matcher>
            = prefix:$(['L'|'R'|'B'] ":")? "(" _ selector:selector() _ ")" {
                Box::new(matchers::GroupMatcher::new(prefix, selector))
            }

        // Expression matcher
        rule expression() -> Box<dyn matchers::Matcher>
            = "-" _ group:group() _ {
                Box::new(matchers::NegateMatcher::new(group))
            }
            / "-" _ path:path() _ {
                Box::new(matchers::NegateMatcher::new(path))
            }
            / group()
            / path()

        // Composite matcher
        rule composite() -> Box<dyn matchers::Matcher>
            = left:expression() _ operator:$(['|' | '&' | '-']) _ right:composite() {
                Box::new(matchers::CompositeMatcher::new(left, operator.chars().next().unwrap(), right))
            }
            / expression()

        // Selector matcher
        rule selector() -> Box<dyn matchers::Matcher>
            = left:composite() _ "," _ right:selector()? {
                if let Some(r) = right {
                    Box::new(matchers::OrMatcher::new(left, r))
                } else {
                    left
                }
            }
            / composite()
    }
}

pub fn parse(selector: &str) -> Result<Box<dyn matchers::Matcher>, ParseError<LineCol>> {
    return parser::parse(selector);
}

pub fn main() {
    match parser::parse("L:a") {
        Ok(selector) => {
            println!("{:?}", selector.get_prefix("b"));
        }
        Err(err) => panic!("Parsing error for selector {}: {}", "L:a", err),
    }
}
// Traits for matchers
pub trait Matcher {
    fn matches(&self, scope: &str) -> bool;
    fn get_prefix(&self, _scopes: &str) -> Option<char> {
        None
    }
}

// SegmentMatcher
pub struct SegmentMatcher {
    segment: String,
}

impl SegmentMatcher {
    pub fn new(segment: &str) -> Self {
        Self { segment: segment.to_string() }
    }
}

impl Matcher for SegmentMatcher {
    fn matches(&self, scope: &str) -> bool {
        scope == self.segment
    }
}

// TrueMatcher
pub struct TrueMatcher;

impl Matcher for TrueMatcher {
    fn matches(&self, _: &str) -> bool {
        true
    }
}

// ScopeMatcher
pub struct ScopeMatcher {
    segments: Vec<Box<dyn Matcher>>,
}

impl ScopeMatcher {
    pub fn new(segments: Vec<Box<(dyn Matcher + 'static)>>) -> Self {
        Self { segments }
    }
}

impl Matcher for ScopeMatcher {
    fn matches(&self, scope: &str) -> bool {
        let scope_segments: Vec<&str> = scope.split('.').collect();
        if scope_segments.len() < self.segments.len() {
            return false;
        }

        self.segments.iter().zip(scope_segments.iter()).all(|(segment, scope)| {
            segment.matches(scope)
        })
    }
}

// GroupMatcher
pub struct GroupMatcher {
    prefix: Option<char>,
    selector: Box<dyn Matcher>,
}

impl GroupMatcher {
    pub fn new(prefix: Option<&str>, selector: Box<dyn Matcher>) -> Self {
        Self {
            prefix: prefix.and_then(|f| f.chars().next()),
            selector,
        }
    }
}

impl Matcher for GroupMatcher {
    fn matches(&self, scopes: &str) -> bool {
        self.selector.matches(scopes)
    }

    fn get_prefix(&self, scopes: &str) -> Option<char> {
        if self.matches(scopes) {
            self.prefix
        } else {
            None
        }
    }
}

// PathMatcher
pub struct PathMatcher {
    prefix: Option<char>,
    matchers: Vec<Box<dyn Matcher>>,
}

impl PathMatcher {
    pub fn new(
        prefix: Option<&str>,
        matchers: Vec<Box<dyn Matcher>>,
    ) -> Self {
        Self {
			prefix: prefix.and_then(|f| f.chars().next()),
			matchers
		}
    }
}

impl Matcher for PathMatcher {
    fn matches(&self, scopes: &str) -> bool {
        let scopes_iter = scopes.split(&[' ', '\t']).peekable();
        let mut index = 0;

        if self.matchers.is_empty() {
            return false;
        }

        let mut matcher = &self.matchers[index];

        for scope in scopes_iter {
            if matcher.matches(scope) {
                index += 1;
                if index >= self.matchers.len() {
                    return true;
                }
                matcher = &self.matchers[index];
            }
        }

        false
    }

    fn get_prefix(&self, scopes: &str) -> Option<char> {
        if self.matches(scopes) {
            self.prefix
        } else {
            None
        }
    }
}

// OrMatcher
pub struct OrMatcher {
    left: Box<dyn Matcher>,
    right: Box<dyn Matcher>,
}

impl OrMatcher {
    pub fn new(left: Box<dyn Matcher>, right: Box<dyn Matcher>) -> Self {
        Self { left, right }
    }
}

impl Matcher for OrMatcher {
    fn matches(&self, scopes: &str) -> bool {
        self.left.matches(scopes) || self.right.matches(scopes)
    }

    fn get_prefix(&self, scopes: &str) -> Option<char> {
        self.left.get_prefix(scopes).or_else(|| self.right.get_prefix(scopes))
    }
}

// AndMatcher
pub struct AndMatcher {
    left: Box<dyn Matcher>,
    right: Box<dyn Matcher>,
}

impl AndMatcher {
    pub fn new(left: Box<dyn Matcher>, right: Box<dyn Matcher>) -> Self {
        Self { left, right }
    }
}

impl Matcher for AndMatcher {
    fn matches(&self, scopes: &str) -> bool {
        self.left.matches(scopes) && self.right.matches(scopes)
    }

    fn get_prefix(&self, scopes: &str) -> Option<char> {
        if self.matches(scopes) {
            self.left.get_prefix(scopes)
        } else {
            None
        }
    }
}

// NegateMatcher
pub struct NegateMatcher {
    matcher: Box<dyn Matcher>,
}

impl NegateMatcher {
    pub fn new(matcher: Box<dyn Matcher>) -> Self {
        Self { matcher }
    }
}

impl Matcher for NegateMatcher {
    fn matches(&self, scopes: &str) -> bool {
        !self.matcher.matches(scopes)
    }
}

// CompositeMatcher
pub struct CompositeMatcher {
    matcher: Box<dyn Matcher>,
}

impl CompositeMatcher {
    pub fn new(
        left: Box<dyn Matcher>,
        operator: char,
        right: Box<dyn Matcher>,
    ) -> Self {
        let matcher: Box<dyn Matcher> = match operator {
            '|' => Box::new(OrMatcher::new(left, right)),
            '&' => Box::new(AndMatcher::new(left, right)),
            '-' => Box::new(AndMatcher::new(left, Box::new(NegateMatcher::new(right)))),
            _ => panic!("Unsupported operator"),
        };
        Self { matcher }
    }
}

impl Matcher for CompositeMatcher {
    fn matches(&self, scopes: &str) -> bool {
        self.matcher.matches(scopes)
    }

    fn get_prefix(&self, scopes: &str) -> Option<char> {
        self.matcher.get_prefix(scopes)
    }
}
#[cfg(test)]
mod tests;
pub mod value;

#[macro_export]
macro_rules! range {
    ($e:expr) => {
        Range::Char($e)
    };
    ($e1:expr, $e2:expr) => {
        Range::Range($e1..=$e2)
    };
}

trait InRange {
    fn in_range(&self, c: char) -> bool;
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Range {
    Char(char),
    Range(std::ops::RangeInclusive<char>),
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Range::Char(c) => write!(f, "{}", c.escape_default()),
            Range::Range(range) => write!(
                f,
                "{}-{}",
                range.start().escape_default(),
                range.end().escape_default()
            ),
        }
    }
}

impl Range {
    pub fn char(c: char) -> Range {
        Range::Char(c)
    }

    pub fn range(range: std::ops::RangeInclusive<char>) -> Range {
        Range::Range(range)
    }
}

impl InRange for Range {
    fn in_range(&self, c: char) -> bool {
        match self {
            Range::Char(c1) => c == *c1,
            Range::Range(range) => range.contains(&c),
        }
    }
}

impl InRange for Vec<Range> {
    fn in_range(&self, c: char) -> bool {
        for r in self {
            if r.in_range(c) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Re {
    Zero,
    One,
    Char(char),
    Range(Vec<Range>),
    Seq(Box<Re>, Box<Re>),
    Alt(Box<Re>, Box<Re>),
    Star(Box<Re>),
    Plus(Box<Re>),
    Optional(Box<Re>),
    Record(String, Box<Re>),
}

impl std::fmt::Display for Re {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Re::Zero => write!(f, "0"),
            Re::One => write!(f, "1"),
            Re::Char(c) => write!(f, "{}", c.escape_default()),
            Re::Range(range) => {
                let mut result = String::new();
                for r in range {
                    result.push_str(r.to_string().as_str());
                }
                write!(f, "[{}]", result)
            }
            Re::Seq(r1, r2) => write!(f, "{}{}", r1, r2),
            Re::Alt(r1, r2) => write!(f, "({}|{})", r1, r2),
            Re::Star(r) => write!(f, "({})*", r),
            Re::Plus(r) => write!(f, "({})+", r),
            Re::Optional(r) => write!(f, "({})?", r),
            Re::Record(name, r) => write!(f, "({}:{})", name, r),
        }
    }
}

impl std::ops::BitAnd for Re {
    type Output = Re;

    fn bitand(self, rhs: Self) -> Self::Output {
        Re::seq(self, rhs)
    }
}

impl std::ops::BitOr for Re {
    type Output = Re;

    fn bitor(self, rhs: Self) -> Self::Output {
        Re::alt(self, rhs)
    }
}

impl Re {
    pub fn seq_from(s: String) -> Re {
        let first = s.chars().next();

        if first.is_none() {
            Re::One
        } else {
            if s.len() == 1 {
                Re::Char(first.unwrap())
            } else {
                Re::Seq(
                    Box::new(Re::Char(first.unwrap())),
                    Box::new(Re::seq_from(s[1..].to_string())),
                )
            }
        }
    }

    pub fn zero() -> Re {
        Re::Zero
    }

    pub fn one() -> Re {
        Re::One
    }

    pub fn char(c: char) -> Re {
        Re::Char(c)
    }

    pub fn range(range: Vec<Range>) -> Re {
        Re::Range(range)
    }

    pub fn seq(r1: Re, r2: Re) -> Re {
        Re::Seq(Box::new(r1), Box::new(r2))
    }

    pub fn alt(r1: Re, r2: Re) -> Re {
        Re::Alt(Box::new(r1), Box::new(r2))
    }

    pub fn star(r: Re) -> Re {
        Re::Star(Box::new(r))
    }

    pub fn plus(r: Re) -> Re {
        Re::Plus(Box::new(r))
    }

    pub fn optional(r: Re) -> Re {
        Re::Optional(Box::new(r))
    }

    pub fn record(name: String, r: Re) -> Re {
        Re::Record(name, Box::new(r))
    }

    pub fn matches(&self, s: &String) -> ExactMatcher {
        ExactMatcher::new(self.clone()).matches(s).clone()
    }

    pub fn matches_prefix(&self, s: &String) -> PrefixMatcher {
        PrefixMatcher::new(self.clone()).matches(s).clone()
    }

    /*
     * not to be used for lexing
     */
    pub(crate) fn simplify(&mut self) -> &Self {
        match self {
            Re::Alt(r1, r2) => {
                r1.simplify();
                r2.simplify();
                match (r1.as_ref(), r2.as_ref()) {
                    (Re::Zero, _) => *self = *r2.to_owned(),
                    (_, Re::Zero) => *self = *r1.to_owned(),
                    (Re::One, r) if r != &Re::One => *self = Re::Optional(Box::new(*r2.to_owned())),
                    (r, Re::One) if r != &Re::One => *self = Re::Optional(Box::new(*r1.to_owned())),
                    (r1s, r2s) if r1s == r2s => *self = *r1.to_owned(),
                    _ => (),
                }
            }
            Re::Seq(r1, r2) => {
                r1.simplify();
                r2.simplify();
                match (r1.as_ref(), r2.as_ref()) {
                    (Re::Zero, _) => *self = Re::Zero,
                    (_, Re::Zero) => *self = Re::Zero,
                    (Re::One, _) => *self = *r2.to_owned(),
                    (_, Re::One) => *self = *r1.to_owned(),
                    _ => (),
                }
            }
            _ => (),
        }

        self
    }

    /*
     * not to be used for lexing
     */
    pub(crate) fn initial_simplify(&mut self) -> Self {
        match self {
            Re::Star(r) => {
                r.simplify();
                match r.as_ref() {
                    Re::Zero => *self = Re::One,
                    Re::One => *self = Re::One,
                    Re::Star(_) => *self = *r.to_owned(),
                    _ => (),
                }
            }
            Re::Alt(r1, r2) => {
                r1.simplify();
                r2.simplify();
                match (r1.as_ref(), r2.as_ref()) {
                    (Re::Zero, _) => *self = *r2.to_owned(),
                    (_, Re::Zero) => *self = *r1.to_owned(),
                    (Re::One, r) if r != &Re::One => *self = Re::Optional(Box::new(*r2.to_owned())),
                    (r, Re::One) if r != &Re::One => *self = Re::Optional(Box::new(*r1.to_owned())),
                    (r1s, r2s) if r1s == r2s => *self = *r1.to_owned(),
                    _ => (),
                }
            }
            Re::Seq(r1, r2) => {
                r1.simplify();
                r2.simplify();
                match (r1.as_ref(), r2.as_ref()) {
                    (Re::Zero, _) => *self = Re::Zero,
                    (_, Re::Zero) => *self = Re::Zero,
                    (Re::One, _) => *self = *r2.to_owned(),
                    (_, Re::One) => *self = *r1.to_owned(),
                    _ => (),
                }
            }
            _ => (),
        }

        self.clone()
    }

    pub(crate) fn nullable(&self) -> bool {
        match self {
            Re::Zero => false,
            Re::One => true,
            Re::Char(_) => false,
            Re::Range(_) => false,
            Re::Seq(r1, r2) => r1.nullable() && r2.nullable(),
            Re::Alt(r1, r2) => r1.nullable() || r2.nullable(),
            Re::Star(_) => true,
            Re::Plus(r) => r.nullable(),
            Re::Optional(_) => true,
            Re::Record(_, r) => r.nullable(),
        }
    }

    pub(crate) fn derivative(&self, c: char) -> Self {
        match self {
            Re::Zero => Re::Zero,
            Re::One => Re::Zero,
            Re::Char(c1) => {
                if *c1 == c {
                    Re::One
                } else {
                    Re::Zero
                }
            }
            Re::Range(r) => {
                if r.in_range(c) {
                    Re::One
                } else {
                    Re::Zero
                }
            }
            Re::Alt(r1, r2) => Re::alt(r1.derivative(c), r2.derivative(c)),
            Re::Seq(r1, r2) => {
                if r1.nullable() {
                    Re::alt(Re::seq(r1.derivative(c), *r2.clone()), r2.derivative(c))
                } else {
                    Re::seq(r1.derivative(c), *r2.clone())
                }
            }
            Re::Star(r) => Re::seq(r.derivative(c), self.clone()),
            Re::Plus(r) => Re::seq(r.derivative(c), Re::Star(r.clone())),
            Re::Optional(r) => r.derivative(c),
            Re::Record(_, r) => r.derivative(c),
        }
    }
}

trait Matcher {
    fn matches(&mut self, s: &String) -> &Self;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactMatcher {
    pub re: Re,
    pub matched: Option<String>,
    pub found: bool,

    pub(crate) used: bool,
}

impl std::fmt::Display for ExactMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.matched {
            Some(matched) => write!(f, "ExactMatcher: re: {}, matched: \"{}\"", self.re, matched),
            None => {
                if self.used {
                    write!(f, "ExactMatcher: re: {}, matched: None", self.re)
                } else {
                    write!(f, "ExactMatcher: re: {}", self.re)
                }
            }
        }
    }
}

impl ExactMatcher {
    pub(crate) fn new(re: Re) -> ExactMatcher {
        ExactMatcher {
            re,
            matched: None,
            found: false,
            used: false,
        }
    }
}

impl Matcher for ExactMatcher {
    fn matches(&mut self, s: &String) -> &Self {
        self.used = true;

        let mut r = self.re.initial_simplify();

        self.matched = None;
        self.found = false;

        for c in s.chars() {
            r = r.derivative(c).simplify().clone();

            if r == Re::Zero {
                self.matched = None;
                self.found = false;
                return self;
            }

            if r.nullable() {
                self.matched = Some(s.clone());
                self.found = true;
            }
        }

        if r.nullable() {
            self.matched = Some(s.clone());
            self.found = true;
        }

        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixMatcher {
    pub re: Re,
    pub matched: Option<String>,
    pub remaining: Option<String>,
    pub found: bool,

    pub(crate) used: bool,
}

impl std::fmt::Display for PrefixMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.matched {
            Some(ref matched) => write!(
                f,
                "PrefixMatcher: re: {}, matched: \"{}\", remaining: \"{}\"",
                self.re,
                matched,
                self.remaining.as_ref().unwrap()
            ),
            None => {
                if self.used {
                    write!(
                        f,
                        "PrefixMatcher: re: {}, matched: None, remaining: None",
                        self.re
                    )
                } else {
                    write!(f, "PrefixMatcher: re: {}", self.re)
                }
            }
        }
    }
}

impl PrefixMatcher {
    pub(crate) fn new(re: Re) -> PrefixMatcher {
        PrefixMatcher {
            re,
            matched: None,
            remaining: None,
            found: false,
            used: false,
        }
    }
}

impl Matcher for PrefixMatcher {
    fn matches(&mut self, s: &String) -> &Self {
        self.used = true;

        if self.re == Re::One {
            self.matched = Some("".to_string());
            self.remaining = Some(s.clone());
            self.found = true;
            return self;
        }

        let mut r = self.re.initial_simplify();

        self.matched = None;
        self.remaining = None;
        self.found = false;

        let mut matched = String::new();

        for c in s.chars() {
            r = r.derivative(c).simplify().clone();
            if r == Re::Zero {
                return self;
            }
            matched.push(c);
            if r.nullable() {
                self.matched = Some(matched.clone());
                self.remaining = Some(s[matched.len()..].to_string());
                self.found = true;
            }
        }

        if r.nullable() {
            self.matched = Some(matched.clone());
            self.remaining = Some(s[matched.len()..].to_string());
            self.found = true;
        }

        self
    }
}

use super::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
    Empty,
    Char(char), // Also used for range
    Seq(Box<Value>, Box<Value>),
    Left(Box<Value>),
    Right(Box<Value>),
    Stars(Vec<Value>), // Also used for optional, and plus
    Record(String, Box<Value>),
}

impl std::ops::BitAnd for Value {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Value::Seq(Box::new(self), Box::new(rhs))
    }
}

impl Value {
    fn flatten(&self) -> String {
        match self {
            Value::Empty => String::new(),
            Value::Char(c) => c.to_string(),
            Value::Seq(v1, v2) => {
                let mut s = v1.flatten();
                s.push_str(&v2.flatten());
                s
            }
            Value::Left(v) => v.flatten(),
            Value::Right(v) => v.flatten(),
            Value::Stars(vs) => {
                let mut s = String::new();
                for v in vs {
                    s.push_str(&v.flatten());
                }
                s
            }
            Value::Record(_, v) => v.flatten(),
        }
    }

    pub(crate) fn environment(&self) -> Vec<(String, String)> {
        match self {
            Value::Empty => Vec::new(),
            Value::Char(_) => Vec::new(),
            Value::Seq(v1, v2) => {
                let mut env = v1.environment();
                env.extend(v2.environment());
                env
            }
            Value::Left(v) => v.environment(),
            Value::Right(v) => v.environment(),
            Value::Stars(vs) => {
                let mut env = Vec::new();
                for v in vs {
                    env.extend(v.environment());
                }
                env
            }
            Value::Record(s, v) => {
                let mut env = v.environment();
                env.push((s.clone(), v.flatten()));
                env
            }
        }
    }
}

impl Re {
    pub(crate) fn make_empty(&self) -> Value {
        match self {
            Re::One => Value::Empty,
            Re::Alt(r1, r2) => {
                if r1.nullable() {
                    Value::Left(Box::new(r1.make_empty()))
                } else {
                    Value::Right(Box::new(r2.make_empty()))
                }
            }
            Re::Seq(r1, r2) => Value::Seq(Box::new(r1.make_empty()), Box::new(r2.make_empty())),
            Re::Star(_) => Value::Stars(Vec::new()),
            Re::Plus(r) => Value::Stars(Vec::from([r.make_empty()])),
            Re::Optional(_) => Value::Stars(Vec::new()),
            Re::Record(name, r) => Value::Record(name.clone(), Box::new(r.make_empty())),
            _ => unreachable!("make_empty called on non nullable Re {:?}", self),
        }
    }

    pub(crate) fn injection(&self, c: char, value: &mut Value) -> Value {
        match (self, value) {
            (Re::Star(r), Value::Seq(v1, st))
                if match **st {
                    Value::Stars(_) => true,
                    _ => false,
                } =>
            {
                let Value::Stars(vs) = st.as_mut() else {
                    unreachable!()
                };
                let mut v = Vec::from([r.injection(c, v1)]);
                v.extend(vs.to_owned());
                Value::Stars(v)
            }
            (Re::Seq(r1, _), Value::Left(l))
                if match **l {
                    Value::Seq(_, _) => true,
                    _ => false,
                } =>
            {
                let Value::Seq(v1, v2) = l.as_mut() else {
                    unreachable!()
                };
                Value::Seq(Box::new(r1.injection(c, v1.as_mut())), v2.to_owned())
            }
            (Re::Seq(r1, _), Value::Seq(v1, v2)) => {
                Value::Seq(Box::new(r1.injection(c, v1.as_mut())), v2.to_owned())
            }
            (Re::Seq(r1, r2), Value::Right(v)) => Value::Seq(
                Box::new(r1.make_empty()),
                Box::new(r2.injection(c, v.as_mut())),
            ),
            (Re::Alt(r1, _), Value::Left(v)) => Value::Left(Box::new(r1.injection(c, v))),
            (Re::Alt(_, r2), Value::Right(v)) => Value::Right(Box::new(r2.injection(c, v))),
            (Re::Char(_), Value::Empty) => Value::Char(c),
            (Re::Range(_), Value::Empty) => Value::Char(c),
            (Re::Plus(r), Value::Seq(v1, st))
                if match **st {
                    Value::Stars(_) => true,
                    _ => false,
                } =>
            {
                let Value::Stars(vs) = st.as_mut() else {
                    unreachable!()
                };
                let mut v = Vec::from([r.injection(c, v1)]);
                v.extend(vs.to_owned());
                Value::Stars(v)
            }
            (Re::Optional(r), v) => {
                let v = r.injection(c, v);
                if v == Value::Empty {
                    Value::Stars(Vec::new())
                } else {
                    Value::Stars(Vec::from([v]))
                }
            }
            (Re::Record(name, r), v) => Value::Record(name.clone(), Box::new(r.injection(c, v))),
            _ => unreachable!("injection called on Re {:?}", self),
        }
    }

    pub(crate) fn simplify_with_rectification(&self) -> (Re, Box<dyn Fn(Value) -> Value>) {
        match self {
            Re::Alt(r1, r2) => {
                let (r1s, f1) = r1.simplify_with_rectification();
                let (r2s, f2) = r2.simplify_with_rectification();
                match (r1s.to_owned(), r2s.to_owned()) {
                    (Re::Zero, simplified_re) => (
                        simplified_re,
                        Box::new(move |v| Value::Right(Box::new(f2(v)))),
                    ),
                    (simplified_re, Re::Zero) => (
                        simplified_re,
                        Box::new(move |v| Value::Left(Box::new(f1(v)))),
                    ),
                    (simplified_re1, simplified_re2) if simplified_re1 == simplified_re2 => (
                        simplified_re1,
                        Box::new(move |v| Value::Left(Box::new(f1(v)))),
                    ),
                    (simplified_re1, simplified_re2) => (
                        Re::Alt(Box::new(simplified_re1), Box::new(simplified_re2)),
                        Box::new(move |v| match v {
                            Value::Left(v) => Value::Left(Box::new(f1(*v))),
                            Value::Right(v) => Value::Right(Box::new(f2(*v))),
                            _ => unreachable!(),
                        }),
                    ),
                }
            }
            Re::Seq(r1, r2) => {
                let (r1s, f1) = r1.simplify_with_rectification();
                let (r2s, f2) = r2.simplify_with_rectification();
                match (r1s.to_owned(), r2s.to_owned()) {
                    (Re::Zero, _) => (Re::Zero, Box::new(|_: Value| -> Value { unreachable!() })),
                    (_, Re::Zero) => (Re::Zero, Box::new(|_: Value| -> Value { unreachable!() })),
                    (Re::One, simplified_re) => (
                        simplified_re,
                        Box::new(move |v| Value::Seq(Box::new(f1(Value::Empty)), Box::new(f2(v)))),
                    ),
                    (simplified_re, Re::One) => (
                        simplified_re,
                        Box::new(move |v| Value::Seq(Box::new(f1(v)), Box::new(f2(Value::Empty)))),
                    ),
                    (simplified_re1, simplified_re2) => (
                        Re::Seq(Box::new(simplified_re1), Box::new(simplified_re2)),
                        Box::new(move |v| match v {
                            Value::Seq(v1, v2) => Value::Seq(Box::new(f1(*v1)), Box::new(f2(*v2))),
                            _ => unreachable!(),
                        }),
                    ),
                }
            }
            _ => (self.clone(), Box::new(move |v| v)),
        }
    }
}

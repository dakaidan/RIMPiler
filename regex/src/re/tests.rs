use std::time::Instant;
use super::super::{*};
use crate::re::{*};

#[test]
fn range_macro() {
    assert_eq!(range!('a'), Range::Char('a'));
    assert_eq!(range!('a', 'z'), Range::Range('a'..='z'));
}

#[test]
fn re_constructors() {
    assert_eq!(Re::zero(), Re::Zero);
    assert_eq!(Re::one(), Re::One);
    assert_eq!(Re::char('a'), Re::Char('a'));
    assert_eq!(Re::range(vec![range!('a')]), Re::Range(vec![Range::Char('a')]));
    assert_eq!(Re::range(vec![range!('a', 'z')]), Re::Range(vec![Range::Range('a'..='z')]));
    assert_eq!(Re::seq(Re::char('a'), Re::char('b')), Re::Seq(Box::new(Re::Char('a')), Box::new(Re::Char('b'))));
    assert_eq!(Re::alt(Re::char('a'), Re::char('b')), Re::Alt(Box::new(Re::Char('a')), Box::new(Re::Char('b'))));
    assert_eq!(Re::star(Re::char('a')), Re::Star(Box::new(Re::Char('a'))));
    assert_eq!(Re::plus(Re::char('a')), Re::Plus(Box::new(Re::Char('a'))));
    assert_eq!(Re::optional(Re::char('a')), Re::Optional(Box::new(Re::Char('a'))));
    assert_eq!(Re::record("a".to_string(), Re::char('a')), Re::Record("a".to_string(), Box::new(Re::Char('a'))));
}

#[test]
fn sequence_from_string() {
    assert_eq!(
        Re::seq_from("abc".to_string()),
        Re::seq(Re::char('a'), Re::seq(Re::char('b'), Re::char('c')))
    );
}

#[test]
fn operator_overloads() {
    assert_eq!(Re::char('a') & Re::char('b'), Re::seq(Re::char('a'), Re::char('b')));
    assert_eq!(Re::char('a') | Re::char('b'), Re::alt(Re::char('a'), Re::char('b')));

    assert_eq!(Re::seq_from("abc".to_string()) & Re::char('d'),
               Re::seq(Re::seq(Re::char('a'), Re::seq(Re::char('b'), Re::char('c'))), Re::char('d'))
    );
    assert_eq!(Re::seq_from("abc".to_string()) | Re::char('d'),
               Re::alt(Re::seq(Re::char('a'), Re::seq(Re::char('b'), Re::char('c'))), Re::char('d'))
    );
}

#[test]
fn re_fmt() {
    assert_eq!(format!("{}", Re::zero()), "0");
    assert_eq!(format!("{}", Re::one()), "1");
    assert_eq!(format!("{}", Re::char('a')), "a");
    assert_eq!(format!("{}", Re::char('\n')), "\\n");
    assert_eq!(format!("{}", Re::char('\\')), "\\\\");
    assert_eq!(format!("{}", Re::char('\'')), "\\'");
    assert_eq!(format!("{}", Re::char('\r')), "\\r");
    assert_eq!(format!("{}", Re::char('\t')), "\\t");
    assert_eq!(format!("{}", Re::range(vec![range!('a')])), "[a]");
    assert_eq!(format!("{}", Re::range(vec![range!('a', 'z')])), "[a-z]");
    assert_eq!(format!("{}", Re::range(vec![range!('a', 'z'), range!('4')])), "[a-z4]");
    assert_eq!(format!("{}", Re::seq(Re::char('a'), Re::char('b'))), "ab");
    assert_eq!(format!("{}", Re::alt(Re::char('a'), Re::char('b'))), "(a|b)");
    assert_eq!(format!("{}", Re::star(Re::char('a'))), "(a)*");
    assert_eq!(format!("{}", Re::plus(Re::char('a'))), "(a)+");
    assert_eq!(format!("{}", Re::optional(Re::char('a'))), "(a)?");
    assert_eq!(format!("{}", Re::optional(Re::alt(Re::char('a'), Re::char('b')))), "((a|b))?");
    assert_eq!(format!("{}", Re::record("a".to_string(), Re::char('a'))), "(a:a)");
}

#[test]
fn re_simplify() {
    assert_eq!(
        Re::alt(Re::zero(), Re::char('a')).simplify(),
        &Re::char('a')
    );

    assert_eq!(
        Re::alt(Re::char('a'), Re::zero()).simplify(),
        &Re::char('a')
    );

    assert_eq!(
        Re::alt(Re::one(), Re::char('a')).simplify(),
        &Re::optional(Re::char('a'))
    );

    assert_eq!(
        Re::alt(Re::char('a'), Re::one()).simplify(),
        &Re::optional(Re::char('a'))
    );

    assert_eq!(
        Re::alt(Re::char('a'), Re::char('a')).simplify(),
        &Re::char('a')
    );

    assert_eq!(
        Re::alt(Re::char('a'), Re::char('b')).simplify(),
        &Re::alt(Re::char('a'), Re::char('b'))
    );

    assert_eq!(
        Re::seq(Re::zero(), Re::char('a')).simplify(),
        &Re::zero()
    );

    assert_eq!(
        Re::seq(Re::char('a'), Re::zero()).simplify(),
        &Re::zero()
    );

    assert_eq!(
        Re::seq(Re::one(), Re::char('a')).simplify(),
        &Re::char('a')
    );

    assert_eq!(
        Re::seq(Re::char('a'), Re::one()).simplify(),
        &Re::char('a')
    );

    assert_eq!(
        Re::seq(Re::char('a'), Re::char('a')).simplify(),
        &Re::seq(Re::char('a'), Re::char('a'))
    );

    assert_eq!(
        Re::alt(
            Re::seq(
                Re::seq_from("abc".to_string()),
                Re::zero()
            ),
            Re::alt(Re::char('a'), Re::char('a'))
        ).simplify(),
        &Re::char('a')
    );
}

#[test]
fn re_initial_simplify() {
    assert_eq!(
        Re::star(Re::zero()).initial_simplify(),
        Re::one()
    );

    assert_eq!(
        Re::star(Re::one()).initial_simplify(),
        Re::one()
    );

    assert_eq!(
        Re::star(Re::star(Re::char('a'))).initial_simplify(),
        Re::star(Re::char('a'))
    )
}

#[test]
fn re_nullable() {
    assert!(!Re::zero().nullable());

    assert!(Re::one().nullable());

    assert!(!Re::char('a').nullable());

    assert!(!Re::range(vec![range!('a', 'z')]).nullable());

    assert!(Re::seq(Re::one(), Re::one()).nullable());
    assert!(!Re::seq(Re::one(), Re::zero()).nullable());
    assert!(!Re::seq(Re::zero(), Re::one()).nullable());
    assert!(!Re::seq(Re::zero(), Re::zero()).nullable());
    assert!(Re::seq(Re::one(), Re::seq(Re::one(), Re::one())).nullable());
    assert!(!Re::seq(Re::one(), Re::char('c')).nullable());

    assert!(Re::alt(Re::one(), Re::one()).nullable());
    assert!(Re::alt(Re::one(), Re::zero()).nullable());
    assert!(Re::alt(Re::zero(), Re::one()).nullable());
    assert!(!Re::alt(Re::zero(), Re::zero()).nullable());
    assert!(Re::alt(Re::one(), Re::alt(Re::one(), Re::one())).nullable());
    assert!(Re::alt(Re::one(), Re::char('c')).nullable());

    assert!(Re::star(Re::one()).nullable());
    assert!(Re::star(Re::zero()).nullable());
    assert!(Re::star(Re::star(Re::one())).nullable());

    assert!(Re::plus(Re::one()).nullable());
    assert!(!Re::plus(Re::zero()).nullable());
    assert!(!Re::plus(Re::char('c')).nullable());

    assert!(Re::optional(Re::one()).nullable());
    assert!(Re::optional(Re::zero()).nullable());
    assert!(Re::optional(Re::char('c')).nullable());

    assert!(Re::record("a".to_string(), Re::one()).nullable());
    assert!(!Re::record("a".to_string(), Re::zero()).nullable());
    assert!(!Re::record("a".to_string(), Re::char('c')).nullable());
}

#[test]
fn re_derivative() {
    assert_eq!(
        Re::zero().derivative('a'),
        Re::zero()
    );

    assert_eq!(
        Re::one().derivative('a'),
        Re::zero()
    );

    assert_eq!(
        Re::char('a').derivative('a'),
        Re::one()
    );
    assert_eq!(
        Re::char('a').derivative('b'),
        Re::zero()
    );

    assert_eq!(
        Re::range(vec![range!('a')]).derivative('a'),
        Re::one()
    );
    assert_eq!(
        Re::range(vec![range!('a')]).derivative('b'),
        Re::zero()
    );
    assert_eq!(
        Re::range(vec![range!('a', 'z')]).derivative('a'),
        Re::one()
    );
    assert_eq!(
        Re::range(vec![range!('a', 'z')]).derivative('b'),
        Re::one()
    );

    assert_eq!(
        Re::alt(Re::char('a'), Re::char('b')).derivative('a'),
        Re::alt(Re::one(), Re::zero())
    );
    assert_eq!(
        Re::alt(Re::char('a'), Re::char('b')).derivative('b'),
        Re::alt(Re::zero(), Re::one())
    );
    assert_eq!(
        Re::alt(Re::char('a'), Re::char('b')).derivative('c'),
        Re::alt(Re::zero(), Re::zero())
    );
    assert_eq!(
        Re::alt(Re::seq_from("abc".to_string()), Re::seq_from("az".to_string())).derivative('a'),
        Re::alt(Re::seq(Re::one(), Re::seq_from("bc".to_string())), Re::seq(Re::one(), Re::seq_from("z".to_string())))
    );
    assert_eq!(
        Re::alt(Re::seq_from("abc".to_string()), Re::seq_from("az".to_string())).derivative('v'),
        Re::alt(Re::seq(Re::zero(), Re::seq_from("bc".to_string())), Re::seq(Re::zero(), Re::seq_from("z".to_string())))
    );

    assert_eq!(
        Re::seq(Re::char('a'), Re::char('b')).derivative('a'),
        Re::seq(Re::one(), Re::char('b'))
    );
    assert_eq!(
        Re::seq(Re::char('a'), Re::char('b')).derivative('b'),
        Re::seq(Re::zero(), Re::char('b'))
    );
    assert_eq!(
        Re::seq(Re::char('a'), Re::char('b')).derivative('c'),
        Re::seq(Re::zero(), Re::char('b'))
    );
    assert_eq!(
        Re::seq(Re::alt(Re::char('a'), Re::one()), Re::char('a')).derivative('a'),
        Re::alt(Re::seq(Re::alt(Re::one(), Re::zero()), Re::char('a')), Re::one())
    );

    assert_eq!(
        Re::star(Re::char('a')).derivative('a'),
        Re::seq(Re::one(), Re::star(Re::char('a')))
    );
    assert_eq!(
        Re::star(Re::char('a')).derivative('b'),
        Re::seq(Re::zero(), Re::star(Re::char('a')))
    );

    assert_eq!(
        Re::plus(Re::char('a')).derivative('a'),
        Re::seq(Re::one(), Re::star(Re::char('a')))
    );
    assert_eq!(
        Re::plus(Re::char('a')).derivative('b'),
        Re::seq(Re::zero(), Re::star(Re::char('a')))
    );

    assert_eq!(
        Re::optional(Re::char('a')).derivative('a'),
        Re::one()
    );
    assert_eq!(
        Re::optional(Re::char('a')).derivative('b'),
        Re::zero()
    );

    assert_eq!(
        Re::record("a".to_string(), Re::char('a')).derivative('a'),
        Re::char('a').derivative('a')
    );
    assert_eq!(
        Re::record("a".to_string(), Re::char('a')).derivative('b'),
        Re::char('a').derivative('b')
    );
}

#[test]
fn re_exact_matcher() {
    assert_eq!(
        Re::char('a').matches(&"a".to_string()),
        ExactMatcher{
            matched: Some("a".to_string()),
            found: true,
            re: Re::char('a'),
            used: true,
        }
    );

    assert_eq!(
        Re::char('a').matches(&"b".to_string()),
        ExactMatcher{
            matched: None,
            found: false,
            re: Re::char('a'),
            used: true,
        }
    );

    assert_eq!(
        Re::seq_from("abcdefg".to_string()).matches(&"abcdefg".to_string()),
        ExactMatcher{
            matched: Some("abcdefg".to_string()),
            found: true,
            re: Re::seq_from("abcdefg".to_string()),
            used: true,
        }
    );

    assert_eq!(
        Re::seq_from("abcdefg".to_string()).matches(&"abcdefghi".to_string()),
        ExactMatcher{
            matched: None,
            found: false,
            re: Re::seq_from("abcdefg".to_string()),
            used: true,
        }
    );

    assert_eq!(
        Re::seq_from("abcdefg".to_string()).matches(&"abc".to_string()),
        ExactMatcher{
            matched: None,
            found: false,
            re: Re::seq_from("abcdefg".to_string()),
            used: true,
        }
    );

    assert_eq!(
        Re::record("a".to_string(), Re::char('a')).matches(&"a".to_string()),
        ExactMatcher{
            matched: Some("a".to_string()),
            found: true,
            re: Re::record("a".to_string(), Re::char('a')),
            used: true,
        }
    )
}

#[test]
fn re_exact_matcher_fmt() {
    assert_eq!(
        format!("{}", ExactMatcher{
            used: false,
            found: false,
            matched: None,
            re: Re::char('a'),
        }),
        "ExactMatcher: re: a"
    );

    assert_eq!(
        format!("{}", ExactMatcher{
            used: true,
            found: false,
            matched: None,
            re: Re::char('a'),
        }),
        "ExactMatcher: re: a, matched: None"
    );

    assert_eq!(
        format!("{}", ExactMatcher{
            used: true,
            found: true,
            matched: Some("a".to_string()),
            re: Re::char('a'),
        }),
        "ExactMatcher: re: a, matched: \"a\""
    );
}

#[test]
fn re_prefix_matcher() {
    assert_eq!(
        Re::char('a').matches_prefix(&"a".to_string()),
        PrefixMatcher{
            matched: Some("a".to_string()),
            remaining: Some("".to_string()),
            found: true,
            re: Re::char('a'),
            used: true,
        }
    );

    assert_eq!(
        Re::char('a').matches_prefix(&"b".to_string()),
        PrefixMatcher{
            matched: None,
            remaining: None,
            found: false,
            re: Re::char('a'),
            used: true,
        }
    );

    assert_eq!(
        Re::char('a').matches_prefix(&"ab".to_string()),
        PrefixMatcher{
            matched: Some("a".to_string()),
            remaining: Some("b".to_string()),
            found: true,
            re: Re::char('a'),
            used: true,
        }
    );

    assert_eq!(
        Re::seq_from("abcdefg".to_string()).matches_prefix(&"abcdefg".to_string()),
        PrefixMatcher{
            matched: Some("abcdefg".to_string()),
            remaining: Some("".to_string()),
            found: true,
            re: Re::seq_from("abcdefg".to_string()),
            used: true,
        }
    );

    assert_eq!(
        Re::seq_from("abcdefg".to_string()).matches_prefix(&"abcdefghi".to_string()),
        PrefixMatcher{
            matched: Some("abcdefg".to_string()),
            remaining: Some("hi".to_string()),
            found: true,
            re: Re::seq_from("abcdefg".to_string()),
            used: true,
        }
    );

    assert_eq!(
        Re::seq_from("abcdefg".to_string()).matches_prefix(&"abc".to_string()),
        PrefixMatcher{
            matched: None,
            remaining: None,
            found: false,
            re: Re::seq_from("abcdefg".to_string()),
            used: true,
        }
    );

    // test that it takes the longest possible prefix
    assert_eq!(
        Re::alt(Re::seq_from("abc".to_string()), Re::seq_from("abcde".to_string())).matches_prefix(&"abc".to_string()),
        PrefixMatcher{
            matched: Some("abc".to_string()),
            remaining: Some("".to_string()),
            found: true,
            re: Re::alt(Re::seq_from("abc".to_string()), Re::seq_from("abcde".to_string())),
            used: true,
        }
    );

    assert_eq!(
        Re::alt(Re::seq_from("abc".to_string()), Re::seq_from("abcde".to_string())).matches_prefix(&"abcde".to_string()),
        PrefixMatcher{
            matched: Some("abcde".to_string()),
            remaining: Some("".to_string()),
            found: true,
            re: Re::alt(Re::seq_from("abc".to_string()), Re::seq_from("abcde".to_string())),
            used: true,
        }
    );

    assert_eq!(
        Re::record("a".to_string(), Re::char('a')).matches_prefix(&"a".to_string()),
        PrefixMatcher{
            matched: Some("a".to_string()),
            remaining: Some("".to_string()),
            found: true,
            re: Re::record("a".to_string(), Re::char('a')),
            used: true,
        }
    )
}

#[test]
fn re_prefix_matcher_fmt() {
    assert_eq!(
        format!("{}", PrefixMatcher{
            used: false,
            found: false,
            matched: None,
            remaining: None,
            re: Re::char('a'),
        }),
        "PrefixMatcher: re: a"
    );

    assert_eq!(
        format!("{}", PrefixMatcher{
            used: true,
            found: false,
            matched: None,
            remaining: None,
            re: Re::char('a'),
        }),
        "PrefixMatcher: re: a, matched: None, remaining: None"
    );

    assert_eq!(
        format!("{}", PrefixMatcher{
            used: true,
            found: true,
            matched: Some("a".to_string()),
            remaining: Some("".to_string()),
            re: Re::char('a'),
        }),
        "PrefixMatcher: re: a, matched: \"a\", remaining: \"\""
    );
    assert_eq!(
        format!("{}", PrefixMatcher{
            used: true,
            found: true,
            matched: Some("a".to_string()),
            remaining: Some("bc".to_string()),
            re: Re::char('a'),
        }),
        "PrefixMatcher: re: a, matched: \"a\", remaining: \"bc\""
    );
}

#[ignore = "this test takes a long time"]
#[test]
fn stress_test() {
    let mut re = Re::alt(Re::plus(Re::char('a')), Re::seq(Re::star(Re::char('a')), Re::optional(Re::char('b'))));

    let now = Instant::now();
    for _ in 0..10 {
        re = Re::star(re);
        for n in 0..100 {
            let result = re.matches(&"a".repeat(10 * n));
            assert!(result.found);
            let result = re.matches_prefix(&"a".repeat(10 * n));
            assert!(result.found);
        }
    }
    println!("{}ms", now.elapsed().as_millis());
}
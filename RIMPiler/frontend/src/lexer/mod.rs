#![warn(non_snake_case)]
#[cfg(test)]
mod tests;
pub mod tokens;

use regex::lexer::{Lexer, TokenMeta};
use regex::re::{Range, Re};
use utilities::debug::Error;

/*
We need a lexer for the following grammar:
    <Program> ::= <Statements>

    <Statement> ::= skip | <type> identifier '=' <ArithmeticExpression>
    | identifier '=' <ArithmeticExpression>
    | 'if' <BooleanExpression> 'then' <Block> 'else' <Block>
    | 'while' <BooleanExpression> 'do' <Block>

    <Statements> ::= <Statement>';'<Statement> | <Statement>;

    <Block> ::= '"{"'<Statements>'"}"' |  {<Statement>;}

    <ArithmeticExpression> ::= <ArithmeticTerm> '+' <ArithmeticExpression>
    | <ArithmeticTerm> '-' <ArithmeticExpression> | <ArithmeticTerm> | -<ArithmeticTerm>

    <ArithmeticTerm> ::= <ArithmeticFactor> '*' <ArithmeticTerm>
    | <ArithmeticFactor> '/' <ArithmeticTerm> | <ArithmeticFactor> '^' <ArithmeticTerm>
    | <ArithmeticFactor>

    <ArithmeticFactor> ::= '('<ArithmeticExpression>')' | number | identifier

    <BooleanExpression> ::= <ArithmeticExpression>'=='<ArithmeticExpression>
    | <ArithmeticExpression>'"<"'<ArithmeticExpression>
    | <ArithmeticExpression>'">"'<ArithmeticExpression>
    | <ArithmeticExpression>!=<ArithmeticExpression> | <BooleanTerm>

    <BooleanTerm> ::= <BooleanFactor> '\&\&'<BooleanExpression>
    | <BooleanFactor>'||'<BooleanExpression> | '!' <BooleanExpression> | <BooleanFactor>

    <BooleanFactor> ::= '('<BooleanExpression>')'

    <type> ::= 'int'
*/

pub struct InitialisationRequired;
pub struct Initialised;

pub struct Tokeniser<T> {
    /*
    Comments can be:
        // (letters | symbols | digits | whitespace)* \n
        /* (letters | symbols | digits | whitespace | newline)* */
    */
    comment: Re,
    /*
    Numbers can be:
        0 | [1-9][0-9]*
     */
    number: Re,
    /*
    Keywords can be:
        skip | if | then | else | while | do | int
     */
    keyword: Re,
    /*
    Identifiers can be:
        [a-zA-Z][a-zA-Z0-9]*
     */
    identifier: Re,
    /*
    BinaryOperators can be:
        + | - | * | / | ^ | = | == | < | > | != | && | || | !
     */
    operator: Re,
    /*
    Semicolons can be:
        ;
     */
    semicolon: Re,
    /*
    Brackets can be:
        (Parentheses | ) | { | }
     */
    brackets: Re,
    /*
    Whitespace can be:
        ' ' | '\n' | '\t' | '\r'
     */
    whitespace: Re,
    /*
    rimp is a record combination of this, ordered such that
    keywords are left of identifiers, so that they take precedence
     */
    rimp: Re,
    phantom: std::marker::PhantomData<T>,
}

impl Tokeniser<InitialisationRequired> {
    pub fn new() -> Self {
        Self {
            comment: (Re::Char('/')
                & Re::Char('/')
                & Re::Star(Box::new(Re::Range(vec![
                    Range::Range(' '..='~'),
                    Range::Char('\t'),
                ])))
                & Re::Char('\n'))
                | (Re::Char('/')
                    & Re::Char('*')
                    & Re::Star(Box::new(Re::Range(vec![
                        Range::Range(' '..='~'),
                        Range::Char('\n'),
                        Range::Char('\t'),
                    ])))
                    & Re::Char('*')
                    & Re::Char('/')),
            number: Re::Char('0')
                | (Re::Range(vec![Range::Range('1'..='9')])
                    & Re::Star(Box::new(Re::Range(vec![Range::Range('0'..='9')])))),
            keyword: Re::seq_from("skip".to_string())
                | Re::seq_from("if".to_string())
                | Re::seq_from("then".to_string())
                | Re::seq_from("else".to_string())
                | Re::seq_from("while".to_string())
                | Re::seq_from("do".to_string())
                | Re::seq_from("int".to_string()),
            identifier: (Re::Range(vec![Range::Range('a'..='z'), Range::Range('A'..='Z')])
                & Re::Star(Box::new(Re::Range(vec![
                    Range::Range('a'..='z'),
                    Range::Range('A'..='Z'),
                    Range::Range('0'..='9'),
                ])))),
            operator: (Re::Range(vec![
                Range::Char('+'),
                Range::Char('-'),
                Range::Char('*'),
                Range::Char('/'),
                Range::Char('^'),
                Range::Char('='),
                Range::Char('<'),
                Range::Char('>'),
                Range::Char('!'),
            ]) | Re::seq_from("!=".to_string())
                | Re::seq_from("==".to_string())
                | Re::seq_from("&&".to_string())
                | Re::seq_from("||".to_string())),
            semicolon: Re::Char(';'),
            brackets: Re::Range(vec![
                Range::Char('('),
                Range::Char(')'),
                Range::Char('{'),
                Range::Char('}'),
            ]),
            whitespace: Re::Range(vec![
                Range::Char(' '),
                Range::Char('\n'),
                Range::Char('\t'),
                Range::Char('\r'),
            ]),
            rimp: Re::Zero,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn initialise(&mut self) -> Tokeniser<Initialised> {
        self.rimp = Re::Plus(Box::new(
            Re::Record(String::from("keyword"), Box::new(self.keyword.clone()))
                | Re::Record(String::from("operator"), Box::new(self.operator.clone()))
                | Re::Record(String::from("bracket"), Box::new(self.brackets.clone()))
                | Re::Record(String::from("semicolon"), Box::new(self.semicolon.clone()))
                | Re::Record(
                    String::from("whitespace"),
                    Box::new(self.whitespace.clone()),
                )
                | Re::Record(String::from("number"), Box::new(self.number.clone()))
                | Re::Record(String::from("comment"), Box::new(self.comment.clone()))
                | Re::Record(
                    String::from("identifier"),
                    Box::new(self.identifier.clone()),
                ),
        ));

        Tokeniser {
            comment: self.comment.clone(),
            number: self.number.clone(),
            keyword: self.keyword.clone(),
            identifier: self.identifier.clone(),
            operator: self.operator.clone(),
            semicolon: self.semicolon.clone(),
            brackets: self.brackets.clone(),
            whitespace: self.whitespace.clone(),
            rimp: self.rimp.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl Tokeniser<Initialised> {
    pub fn tokenise(&mut self, input: String) -> Result<Vec<TokenMeta<tokens::RIMPToken>>, Error> {
        let lexer = Lexer::new(self.rimp.to_owned());

        let result = lexer.tokenise::<tokens::RIMPToken>(&input);

        match result {
            Ok(tokens) => Ok(tokens
                .into_iter()
                .filter(|token| match token.token {
                    tokens::RIMPToken::Whitespace => false,
                    tokens::RIMPToken::Comment => false,
                    _ => true,
                })
                .collect()),
            Err(error) => Err(
                Error::from_error(
                    error,
                    String::from("Failed to tokenise to RIMP tokens"),
                    String::from("Tokeniser"),
                )
            )
        }
    }

    pub fn tokenise_without_filtering(
        &mut self,
        input: String,
    ) -> Result<Vec<TokenMeta<tokens::RIMPToken>>, Error> {
        let lexer = Lexer::new(self.rimp.to_owned());

        let result = lexer.tokenise::<tokens::RIMPToken>(&input);

        match result {
            Ok(tokens) => Ok(tokens),
            Err(error) => Err(
                Error::from_error(
                    error,
                    String::from("Failed to tokenise to RIMP tokens"),
                    String::from("Tokeniser"),
                )
            )
        }
    }
}

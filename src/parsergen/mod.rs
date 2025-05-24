struct Parser {
    rules: Vec<Rule>,
}

struct Rule {
    name: String,
}

struct Sequence {
    symbols: Vec<Symbol>,
}

enum Symbol {
    Keyword(String),
    Rule(String),

    Repeat(Vec<Symbol>),
}

impl Parser {

}

/*
let mut parser_gen: ParserGenerator = ParserGenerator::new();

let if_stmt = Rule::new("while", {
    Sequence::new({ Symbol::Token(Token::If), Symbol::Rule("expr"), Symbol::Token(Token::Indent),
        Symbol::Repeat(Symbol::Rule("stmt")), Symbol::Token(Token::Dedent) }),
});

parser_gen.add_rule(if_stmt);

let parser: Parser = parser_gen.compile();
parser.parse(input);
*/
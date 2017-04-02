use parse;
use token;

struct Parser<'stream> {
    stream: &'stream Vec<token::Token>,
    ptr   : usize                ,
}

impl<'stream> Parser<'stream> {
    fn new(stream: &'stream Vec<token::Token>) -> Parser {
        Parser {stream: stream, ptr: 0}
    }

    fn check_n(&self, n: usize) -> Option<&'stream token::Token> {
        self.stream.get(self.ptr + n)
    }

    fn check(&self) -> Option<&'stream token::Token> {
        self.check_n(1)
    }

    fn grab(&mut self) {
        self.ptr += 1;
    }
}

fn parse_expression<'stream>(parser: &mut Parser<'stream>, parent: &mut parse::Node) {

}

fn parse_statement<'stream>(parser: &mut Parser<'stream>, parent: &mut parse::Node) {
    // Parse expression, then expect semicolon.
}

fn parse_block_body<'stream>(parser: &mut Parser<'stream>, parent: &mut parse::Node) {

}

fn parse_define<'stream>(parser: &mut Parser<'stream>, parent: &mut parse::Node) {
    if let Some(def) = parser.check() {
        if let token::TokenValue::Name(ref val) = (*def).value {
            if val != "define" {
                return;
            }
            else{
                parser.grab();
            }
        }
        else {
            return;
        }

        if let Some(name_token) = parser.check_n(2) {
            if let token::TokenValue::Name(ref name) = (*name_token).value {
                parser.grab();
                let value = parse::PrimaryValue::Name(name.to_owned());
                let info  = parse::BlockInfo::Define {table: value}   ;
                let data  = parse::NodeData::Block   {info : info }   ;
                parent.new_child(data);
            }
            else {
                return;
            }
        }
        else {
            return;
        }
    }
    else {
        return;
    }   
}

fn parse_method<'stream>(parser: &mut Parser<'stream>, parent: &mut parse::Node) {
    // Parse Block Body
}

fn parse_method_call<'stream>(parser: &mut Parser<'stream>, parent: &mut parse::Node) {
    if let Some(token) = parser.check() {
        if let token::TokenValue::Name (ref n) = (*token).value {
            let mut call = parse::Node {
                data: parse::NodeData::Call {
                    method: parse::PrimaryValue::Name( n.to_owned() )
                },
                children: Vec::new(),
            };

            parser.grab();
            
            loop {
                if let Some(token) = parser.check() {
                    if let token::TokenValue::Semicolon = (*token).value {
                        break
                    }
                    parse_expression(parser, &mut call);
                }
                else {
                    break;
                }
            }

            parent.add_child(call);
        }
    }
}

fn parse_program(parser: &mut Parser, tree: &mut parse::Tree) {
    loop {
        let mut root = tree.root();
        if let Some(token) = parser.check() {
            match (*token).value {
                token::TokenValue::Name (ref n) => {
                    if n == "define" {
                        parse_define(parser, &mut root);
                    }
                    else {
                        if let Some(next) = parser.check_n(2) {
                            match (*next).value {
                                token::TokenValue::Name (_) => { parse_method_call(parser, &mut root) },
                                _        => {},
                            }
                        }
                    }
                },

                token::TokenValue::Bang => {
                    parse_method(parser, &mut root);
                },

                _                       => {
                    parse_statement(parser, &mut root);
                },
            }
            println!("{:?}", token);
        }
        else {
            break;
        }
    }
}

pub fn parse(stream: &Vec<token::Token>) -> parse::Tree {
    let mut tree = parse::Tree::new();
    let mut parser = Parser::new(stream);
    parse_program(&mut parser, &mut tree);
    tree.walk();
    tree
}



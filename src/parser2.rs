struct Parser<'s>  {
    stream: &'s Vec<token::Token> ,
    last_ptr  : Option<usize>     ,
    cur_ptr   : Option<usize>     ,
    next_ptr  : Option<usize>     ,
}

impl<'s> Parse <'s> {
    fn new(stream: &'s Vec<token::Token>) -> Parser<'s> {
        Parser {stream  : stream  ,
                prev_ptr: None    ,
                cur_ptr : Some(0) ,
                next_ptr: Some(1) ,}
    }

    fn current(&self) -> Option<&'s token::Token> {
        if let Some(cur) = self.cur_ptr {
            return self.stream.get(self.cur_ptr);
        }
        else {
            return None;
        }
    }

    fn previous(&self) -> Option<&'s token::Token> {
        if let Some(prev) = self.prev_ptr {
            return self.stream.get(self.prev_ptr);
        }
        else {
            return None;
        }
    }

    fn next(&self) -> Option<&'s token::Token> {
        if let Some(next) = self.next_ptr {
            return self.stream.get(self.next_ptr);
        }
        else {
            return None;
        }
    }

    fn tick(&mut self) {
        if let Some(cur) = self.cur_ptr {
            self.cur_ptr  = Some(cur + 1);
            self.next_ptr = Some(cur + 2);
            self.prev_ptr = Some(cur    );
        }
    }

    fn grab(&mut self) {
        self.tick();
        self.current()
    }
}

fn parse_program<'s>(parser: &mut Parser<'s>, parent: &mut parse::Node) {
    
}

pub fn parse(stream: &Vec<token::Token>) -> parse::Tree {
    let mut tree = parse::Tree::new();
    let mut parser = Parser::new(stream);
    parse_program(&mut parser, tree.root());
    tree.walk();
    tree
}

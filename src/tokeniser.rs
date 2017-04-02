use token;

struct Region {
    start_index : usize,
    start_row   : u64,
    start_col   : u64,
}

struct Consumer<'a> {
    slice : &'a str  ,
    chars : Vec<char>,
    ptr   : usize    ,
    grab  : usize    ,

    row    : u64     ,
    column : u64     ,

    region : Option<Region>,
}

#[derive(PartialEq, Eq)]
enum Literal {
    Undetermined ,
    Float        ,
    Integer      ,
    Name         ,
}

fn is_name_breaking(c: char) -> bool {
    if !(c.is_alphabetic() || c.is_numeric() || c == '_'){
        return true;
    }
    false
}

fn is_numeric_breaking(c: char) -> bool {
    if !(c.is_digit(10) || c == '.') {
        return true;
    }
    false    
}

fn placeholder_origin() -> token::Origin {
   token::Origin {index: 0, row: 0, column: 0, length: 0} 
}

impl<'a> Consumer<'a> {
    fn new(slice: &'a str) -> Consumer {
        let chars  = slice.chars().collect();
        Consumer {
                  slice  : slice,
                  chars  : chars,
                  ptr    : 0    ,
                  grab   : 0    ,

                  row    : 1    ,
                  column : 1    ,

                  region : None ,
        }
    }

    fn region_start(&mut self) {
        self.region = Some( Region {
            start_index: self.ptr,
            start_row  : self.row,
            start_col  : self.column,
        });
    }
    
    fn region_end(&mut self) -> Option<token::Origin> {
        let mut origin = None;
        if let Some(ref region) = self.region {
            let length = (self.ptr - region.start_index) as u64;
            let region_orig = token::Origin {
                                 index  : region.start_index as u64 ,
                                 row    : region.start_row          ,
                                 column : region.start_col          ,
                                 length : length                    ,
                                };
            origin = Some(region_orig);
        }

        self.region = None;
        origin
    }

    fn current_char(&'a self) -> Option<&'a char> {
        return self.chars.get(self.ptr);
    }
 
    fn next_char(&'a self) -> Option<&'a char> {
        return self.chars.get(self.ptr + self.grab + 1);
    }

    fn current(&self) -> Option<char> {
        if let Some(c) = self.current_char() {
            return Some(*c);
        }
        else {
            return None;
        }    
    }

    fn peek(&self) -> Option<char> {
        if let Some(c) = self.next_char() {
            return Some(*c);
        }
        else {
            return None;
        }
    }
 
    fn grab(&mut self) {
        self.grab += 1;
    }


    fn ungrab(&mut self) -> () {
        self.grab = 0;
    }

    fn consume(&mut self) -> String {
        let mut builder : Vec<char> = Vec::new();
        for index in self.ptr .. (self.ptr + self.grab + 1) {
            if let Some(c) = self.chars.get(index) {
                if *c == '\n' {
                    self.row    += 1;
                    self.column  = 1;
                }
                else {
                    self.column += 1;
                }
                builder.push(*c);
            }
            else {
                break
            }
        }
        self.ptr = self.ptr + self.grab + 1;
        self.grab = 0;
        builder.into_iter().collect()
    }

    fn consume_while<C>(&mut self, condition: C) -> String
        where C: Fn(char) -> bool {
            if let Some(cur) = self.current() {
                if !condition(cur) {
                    return String::new();
                }
            }
            else {
                return String::new();
            }

            loop {
                if let Some(peek) = self.peek() {
                    if !condition(peek) {
                        return self.consume();
                    }
                    else {
                        self.grab();
                    }
                }
                else {
                    return self.consume();
                }
            }
    }

    fn consume_value(&mut self) -> Option<token::Token> {
        let mut ltype = Literal::Undetermined;

        if let Some(cur) = self.current() {
            if is_name_breaking(cur) && is_numeric_breaking(cur) {
                return None;
            }
        }
        else {
            return None;
        }

        self.region_start();

        loop {
            if let Some(peek) = self.peek() {
                if is_name_breaking(peek) && is_numeric_breaking(peek) {
                    // If it breaks names and numbers, we break.
                    if ltype == Literal::Undetermined {
                        ltype = Literal::Integer;

                        let value = self.consume();
                        if let Ok(ival) = value.parse::<i64>() {
                            let number = token::Number::Integer(ival);
                            if let Some(token_origin) = self.region_end() {
                                return Some(token::Token {origin: token_origin,
                                                          value : token::TokenValue::NumberLiteral(number)});
                            }
                        }
                        else {
                            return None;
                        }
                    }
                    else if ltype == Literal::Name {
                        let name = self.consume();
                        if let Some(token_origin) = self.region_end() {
                            return Some(token::Token {origin: token_origin,
                                                      value : token::TokenValue::Name(name)});
                        }
                    }
                    else if ltype == Literal::Float {
                        let value = self.consume();
                        if let Ok(fval) = value.parse::<f64>() {
                            let number = token::Number::Float(fval);
                            if let Some(token_origin) = self.region_end() {
                                return Some(token::Token {origin: token_origin,
                                                          value : token::TokenValue::NumberLiteral(number)});
                            }
                        }
                        else {
                            return None;
                        }
                    }
                }
                else if is_numeric_breaking(peek) {
                    // If it breaks numeric, it must be a Name
                    if ltype == Literal::Undetermined {
                        ltype = Literal::Name;
                        self.grab();
                    }
                    else if ltype == Literal::Name {
                        self.grab();
                    }
                    else if ltype == Literal::Float {
                        // It's a float; we break.
                        let value = self.consume();
                        if let Ok(fval) = value.parse::<f64>() {
                            let number = token::Number::Float(fval);
                            if let Some(token_origin) = self.region_end() {
                                return Some(token::Token {origin: token_origin,
                                                          value : token::TokenValue::NumberLiteral(number)});
                            }
                        }
                        else {
                            return None;
                        }
                    }
                }
                else if is_name_breaking(peek) {
                    // We always expect this to be a "."
                    // This is just to protect against future stupidity
                    let point = peek == '.';
                    if ltype == Literal::Undetermined {
                        if point {
                            ltype = Literal::Float;
                            self.grab()
                        }
                    }
                    else if ltype == Literal::Float {
                        if point {
                            // Two decimal points?!
                            return None; 
                        }
                    }
                    else {
                        let name = self.consume();
                        if let Some(token_origin) = self.region_end() {
                            return Some(token::Token {origin: token_origin,
                                                      value : token::TokenValue::Name(name)});
                        }
                    }
                }
                else {
                    // If it breaks nothing we just grab
                    // (Also, that means it's a digit)
                    self.grab();
                }
            }
            else {
                return None;
            }
        }
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(|c| c.is_whitespace())
    }

    fn consume_string_literal(&mut self) -> Option<token::Token> {
        if let Some(init) = self.current() {
            if !(init == '\"') {
                return None;
            }
            else {
                self.region_start();

                let mut escape = false;
                let mut controls: Vec<u32> = vec![0];
                let mut index = 1;
                loop {
                    if let Some(c) = self.peek() {
                        if !escape {
                            if c == '\"' {
                                let mut result: Vec<char> = self.consume().chars().collect();
                                
                                let mut shrink: u32 = 0;
                                for c in &controls {
                                    result.remove((c - shrink) as usize);
                                    shrink += 1;
                                }
                                
                                self.consume();
                                if let Some(origin) = self.region_end(){
                                    let value  = result.into_iter().collect();
                                    return Some(token::Token {origin: origin,
                                                              value : token::TokenValue::StringLiteral(value)});
                                }
                                else {
                                    return None;
                                }
                            }

                            if c == '\\' {
                                escape = true;
                                controls.push(index);
                            }
                            
                        }
                        else {
                            escape = false;
                        }

                        self.grab();
                    }
                    else {
                        return None;
                    }
                    
                    index += 1;
                }

            }
        }
        else {
            return None;
        }
    }

    fn consume_symbol(&mut self) -> Option<token::Token> {
        if let Some(init) = self.current() {
            self.region_start();
            if init == '!' {
                if let Some(next) = self.peek() {
                    if next == '=' {
                        self.grab(); self.consume();
                        return Some(token::Token {origin: self.region_end().unwrap(),
                                                  value : token::TokenValue::NotEqual}); 
                    }
                }
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Bang});
            }

            if init == '#' {
                self.consume();

                if let Some(c) = self.current() {
                    if is_name_breaking(c) {
                        return None;
                    }
                }
                else {
                    return None;
                }

                loop {
                    if let Some(c) = self.peek() {
                        if !is_name_breaking(c) {
                            self.grab();
                        }
                        else {
                            break;
                        }
                    }
                }

                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Hash(self.consume())});

            }

            if init == '{' {
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::LeftBrace}); 
                // LeftBrace
            }

            if init == '}' {
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::RightBrace});
                // RightBrace
            }

            if init == ';' {
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Semicolon});
                // Semicolon
            }

            if init == '.' {
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Dot})    
                // Dot
            }

            if init == '~' {
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Tilde})
                // Tilde
            }

            if init == '-' {
                if let Some(next) = self.peek() {
                    if next == '>' {
                        self.grab(); self.consume();
                        return Some(token::Token {origin: self.region_end().unwrap(),
                                                  value : token::TokenValue::Map}); 
                    }
                }
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Minus});
            }
            if init == ':' {
                if let Some(next) = self.peek() {
                    if next == ':' {
                        self.grab(); self.consume();
                        return Some(token::Token {origin: self.region_end().unwrap(),
                                                  value : token::TokenValue::Assign}); 
                    }
                }
                self.consume();
                return None;
            }
            
            if init == '=' {
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Equal})
                // Equal
            }

            if init == '>' {
                if let Some(next) = self.peek() {
                    if next == '=' {
                        self.grab(); self.consume();
                        return Some(token::Token {origin: self.region_end().unwrap(),
                                                  value : token::TokenValue::GreaterOrEqual}); 
                    }
                }
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Greater});
            }
            
            if init == '<' {
               if let Some(next) = self.peek() {
                    if next == '=' {
                        self.grab(); self.consume();
                        return Some(token::Token {origin: self.region_end().unwrap(),
                                                  value : token::TokenValue::LessOrEqual}); 
                    }
                }
                self.consume();
                return Some(token::Token {origin: self.region_end().unwrap(),
                                          value : token::TokenValue::Less});
            }

            return None;
        }

        None
    }

}



pub fn tokenise(program: &str) -> Vec<token::Token> {
    let mut consumer = Consumer::new(program);
    let mut tokens   = Vec::new();
    loop {
        if let Some(c) = consumer.current() {
            if c.is_whitespace() {
                consumer.consume_whitespace();
            }
            else if !(is_name_breaking(c) && is_numeric_breaking(c)) {
                let value = consumer.consume_value();
                if let Some(token) = value {
                    tokens.push(token);
                }
                else {
                    println!("Failed to parse Value");
                    break;
                }
            }
            else if c == '\"' {
                if let Some(token) = consumer.consume_string_literal(){
                    tokens.push(token);
                }
            }
            else { 
                if let Some(symbol) = consumer.consume_symbol() {
                    tokens.push(symbol);
                }
            }
        }
        else {
            break;
        }
    }

    tokens
}

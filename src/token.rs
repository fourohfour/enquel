#[derive(Debug)]
pub struct Origin {
    pub row_index : i32,
    pub col_index : i32,
    pub length    : i32,
}

#[derive(Debug)]
pub enum Number {
    Integer(i64) ,
    Float(f64)   ,
}

#[derive(Debug)]
pub enum TokenValue {
    // Data Tokens
    Name          (String) ,
    StringLiteral (String) ,
    NumberLiteral (Number) ,

    // Semantic and Structural
    Bang        ,
    Hash(String),

    LeftBrace   ,
    RightBrace  ,
    
    Semicolon   ,
    Dot         ,
    Tilde       ,
    
    // Unarys Ops
    Minus       ,

    // Binary Ops
    Assign         ,
    Map            ,

    Equal          ,
    NotEqual       ,
    Greater        ,
    Less           ,
    GreaterOrEqual ,
    LessOrEqual    ,
}

#[derive(Debug)]
pub struct Token {
    pub origin : Origin    ,
    pub value  : TokenValue,
}

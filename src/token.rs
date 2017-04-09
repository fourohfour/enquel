#[derive(Debug)]
pub struct Origin {
    pub index     : u64,
    pub row       : u64,
    pub column    : u64,
    pub length    : u64,
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
    DoubleColon    ,
    SetEqual       ,
    Arrow          ,

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

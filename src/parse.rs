use token;
pub use token::Number;

#[derive(Debug)]
pub enum ArgClass {
    Modify,
    Map   ,
    Pivot ,
}

#[derive(Debug)]
pub enum LogicalBlock {
    Any    ,
    All    ,
    Either ,
    Neither,
}


#[derive(Debug)]
pub enum EvaluatorType {
    DoDrop,
    Warn  ,
    Panic ,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Equal          ,
    NotEqual       ,
    Greater        ,
    Less           ,
    GreaterOrEqual ,
    LessOrEqual    ,
}

#[derive(Debug)]
pub enum NodeData {
    Root,

    // Precedence 1 (Primary)
    StringLiteral { value: String } ,
    NumberLiteral { value: Number } ,
    Name          { value: String } ,

    // Precedence 2 (Tightest)
    Evaluator { eval:
                EvaluatorType } ,
    Embed                       ,
    Scope                       ,
    Accessor                    ,

    // Precedence 3
    Minus                       ,
    
    // Precedence 4
    Comparison { operator:
           ComparisonOperator } ,

    Assignment                  ,
    Map                         ,

    // Precedence 4
    Args    { class: ArgClass } ,
    Call    { method: String }  ,
    Action  { name  : String }  ,
    
    // Precedence 5 (Structural)
    Define  { table: String }   ,
    Index   { table: String }   ,
    Method  { table: String }   ,
    Logical { operator:
              LogicalBlock  }   ,
   
    // Precedence 6
    Block                       ,
    Statement                   ,
}

pub struct Node {
    pub data     : NodeData      ,
    pub children : Vec<Node>     ,
}

impl Node {
    pub fn add_child(&mut self, c : Node) {
        self.children.push(c);
    }

    pub fn new_child(&mut self, data: NodeData) {
        let node = Node {data: data, children: Vec::new()};
        self.add_child(node);
    }

    fn walk_display(&self, depth: u32) {
        println!("{:?}", self.data);
        let new_depth = depth + 1;
        for child in &self.children {
            child.walk_display(new_depth);
        }
    }
}

pub struct Tree {
    root : Node,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root : Node {
                data    : NodeData::Root ,
                children: Vec::new()     ,
            },
        }
    }

    pub fn root<'rnode>(&'rnode mut self) -> &'rnode mut Node {
        &mut self.root
    }

    pub fn walk<'rnode>(&self) {
        self.root.walk_display(0);
    }
}


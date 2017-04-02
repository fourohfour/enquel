use token;
pub use token::Number;

#[derive(Debug)]
pub enum PrimaryValue {
    Name          (String),
    StringLiteral (String),
    NumberLiteral (Number),
}

#[derive(Debug)]
pub enum LogicalBlock {
    Any    ,
    All    ,
    Either ,
    Neither,
}

#[derive(Debug)]
pub enum BlockInfo {
    Define  {table: PrimaryValue   },
    
    Index   {table: PrimaryValue   },

    Method  {name : PrimaryValue,
             table: PrimaryValue,
             eval : PrimaryValue,  },
    
    Logical {operator: LogicalBlock},
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
    Root                               ,
    Primary    { value: PrimaryValue  },
    Evaluator  { eval : EvaluatorType },
    Embed                              ,
    Accessor                           ,
    Minus                              ,
    Comparison { operator:
                 ComparisonOperator   },
    Assignment                         ,
    Map                                ,

    Call       { method: PrimaryValue },
    Block      { info  : BlockInfo    },
    Statement                          ,
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


use std::fmt::Debug;



pub trait Alphabet: Clone + Debug{ }

#[derive(Clone, Debug)]
enum Op<A: Alphabet, E: Clone>{
    Operation(E),
    Literal(A)
}

#[derive(Clone, Debug)]
struct Node<A: Alphabet, E: Clone>
{
    op : Op<A, E>
}

impl<A: Alphabet, E: Clone> Node<A, E>
{
    fn new(op : Op<A, E>) -> Node<A, E>
    {
        Node{
            op 
        }
    }
}

type NodeId = usize;

#[derive(Clone, Debug)]
enum RegexOp
{
    Alternation(NodeId, NodeId),
    Concatenation(NodeId, NodeId),
    Closure(NodeId)
}

type Stack<T> = Vec<T>;

#[derive(Debug)]
pub struct AreaEx<A: Alphabet>
{
    nodes : Vec<Node<A, RegexOp>>,
    stack : Vec<NodeId>,
    root : Option<NodeId>,
}

impl<A: Alphabet> AreaEx<A>
{
    pub fn new() -> AreaEx<A>{
        AreaEx { 
            nodes : Vec::new(),
            stack : Stack::new(),
            root : None
        }
    }

    pub fn dir(&mut self, dir : A) -> &mut AreaEx<A>{
        
        let node = Node::<A, RegexOp>::new(Op::Literal(dir));

        let id = self.push_node(node);

        if self.stack.is_empty(){
            self.stack.push(id);
            self.root = Some(id);
        }
        else 
        {
            let last_id = self.stack.pop().unwrap();
            let mut last = self.get_node(last_id).unwrap().clone();
            
            match &mut last.op{
                Op::Literal(_literal) =>{
                    let and = Node::<A, RegexOp>::new(Op::Operation(RegexOp::Concatenation(last_id, id)));                    
                    let and_id = self.push_node(and);
                    self.root = Some(and_id);

                    self.stack.push(and_id);
                }
                Op::Operation(op) =>{
                    match op{
                        RegexOp::Concatenation(_left, right) =>{
                            let and = Node::<A, RegexOp>::new(Op::Operation(RegexOp::Concatenation(*right, id)));
                            let and_id = self.push_node(and);

                            *right = and_id;

                            self.set_node(last_id, last);
                            self.stack.push(and_id);
                        }
                        _ =>{

                        }
                    }
                }
            }
        }

        return self;
    }

    pub fn print(&self)
    {
        println!("Root node is {}", self.root.unwrap());
        println!("Vec is {:?}", self.nodes);
    }

    fn get_node(&mut self, id : NodeId) -> Option<&mut Node<A, RegexOp>>
    {
        return self.nodes.get_mut(id);
    }

    fn set_node(&mut self, id : NodeId, node : Node<A, RegexOp>)
    {
        self.nodes[id] = node;
    }

    fn push_node(&mut self, node : Node<A, RegexOp>) -> NodeId
    {
        self.nodes.push(node);

        return self.nodes.len() - 1;
    }
}


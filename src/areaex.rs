use std::{process::Child, collections::VecDeque};


enum Op<T, E>{
    Operation(E),
    Literal(T)
}

type ChildNode<T, E> = Option<Box<Node<T, E>>>;

struct Node<T, E>
{
    op : Op<T, E>,
    left : ChildNode<T, E>,
    pub right : ChildNode<T, E>
}

impl<T, E> Node<T, E>
{
    fn new(op : Op<T, E>, left : ChildNode<T, E>, right : ChildNode<T, E>) -> ChildNode<T, E>{
        Some(Box::new(Node::<T, E>{
            op,
            left,
            right
        }))
    }

    fn new_literal(literal : T) -> ChildNode<T, E>
    {
        Some(Box::new(Node::<T, E>{
            op : Op::Literal(literal),
            left : None,
            right : None
        }))
    }
}

enum RegexOp
{
    Alternation,
    Concatenation,
    Closure
}

struct Tree<T, E>
{
    head : ChildNode<T, E>
}

impl<T, E> Tree<T, E>
{
    fn new() -> Tree<T, E>{
        Tree { 
            head: None 
        }
    }
}

type Stack<T> = VecDeque<T>;

pub struct AreaEx<T>
{
    stack : Stack<ChildNode<T, RegexOp>>
}

impl<T> AreaEx<T>
{
    pub fn new() -> AreaEx<T>{
        AreaEx { 
            stack : Stack::new()
        }
    }

    pub fn dir(&mut self, dir : T) -> (){
        
        if self.stack.is_empty()
        {
            let node : ChildNode<T, RegexOp> = Node::new_literal(dir);
            self.stack.push_back(node);
        }
        else
        {
            let mut popped = self.stack.pop_back().unwrap();
            let right : ChildNode<T, RegexOp> = Node::new_literal(dir);

            if let Some(popped) = popped
            {
                match popped.op {
                    Op::Operation(e) => {
                        let concat = Node::new(Op::Operation(RegexOp::Concatenation), popped.right, right);
                        popped.right = concat;
                        self.stack.push_back(Some(popped));
                    }
                    Op::Literal(l) =>{

                    }
                }
            }

            let concat = Node::new(Op::Operation(RegexOp::Concatenation), left, right);
            self.stack.push_back(concat);
        }
    }
}


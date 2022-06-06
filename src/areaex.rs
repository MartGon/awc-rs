use std::{collections::VecDeque, cell::RefCell, rc::Rc};


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

pub struct AreaEx<'a, T>
{
    // Use indices to a Vec or HashMap instead of pointers

    root : ChildNode<T, RegexOp>,
    stack : Stack<ChildNode<T, RegexOp>>,
    focus : Option<&'a mut ChildNode<T, RegexOp>>
}

impl<'a, T> AreaEx<'a, T>
{
    pub fn new() -> AreaEx<'a, T>{
        AreaEx { 
            root : None,
            stack : Stack::new(),
            focus : None
        }
    }

    pub fn dir(&'a mut self, dir : T) -> (){
        
        if self.root.is_none()
        {
            let node : ChildNode<T, RegexOp> = Node::new_literal(dir);
            self.root = node;
            self.focus = Some(&mut self.root);
        }
        else
        {
            let focus = self.focus.as_mut().unwrap().as_mut().unwrap();
            let op = &focus.op;
            match op{
                Op::Literal(literal) =>{
                    let right : ChildNode<T, RegexOp> = Node::new_literal(dir);
                    let left = Node::new_literal(literal);
                    let and = Node::new(Op::Operation(RegexOp::Concatenation), left, right);
                }
                Op::Operation(op) =>{

                }
            }
        }
    }
}


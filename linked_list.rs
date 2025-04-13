

enum MyLinkedList<T> {
    Node( T , Box<MyLinkedList<T>>),
    Nil,
}


fn print_value<T: std::fmt::Display>(value: T) {
    print!("{}", value);
}



impl<T : std::fmt::Display> MyLinkedList<T> {

    fn new() -> Self{
        MyLinkedList::Nil
    }

    fn append(&mut self,value : T) {
        match self{
            MyLinkedList::Nil => {
                *self = MyLinkedList::Node(value , Box::new(MyLinkedList::Nil))
            },
            MyLinkedList::Node(_, next_node) => {
                next_node.append(value)
            },
        }
    }
    fn prepend(&mut self,value : T) {
        match self{
            MyLinkedList::Nil => {
                *self = MyLinkedList::Node(value , Box::new(MyLinkedList::Nil))
            },
            MyLinkedList::Node(_val, _nod) => {
                *self = MyLinkedList::Node(value, Box::new(std::mem::replace(self, MyLinkedList::Nil)));
    }        }
    }


    fn show(&self) -> (){
        match self{
            MyLinkedList::Nil => {
                print!("Nil");
                println!()
            },
            MyLinkedList::Node(node_val, next_node) => {
                print_value(node_val);
                print!(" -> ");
                next_node.show()
            },
        }
        
    }


}


use crate::MyLinkedList::{ Node, Nil};

fn main() {

    let mut first_linked_list : MyLinkedList<u32> = MyLinkedList::new();
    
    first_linked_list.append(1);
    first_linked_list.append(2);
    first_linked_list.append(3);
    first_linked_list.prepend(0);
    first_linked_list.prepend(0);
    first_linked_list.show()


}

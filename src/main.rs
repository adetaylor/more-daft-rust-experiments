//use std::ops::Add;
use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>
}

/*impl Clone for Node<T>
{
    fn clone(&self) -> Node {
        Node { elem: self.elem.clone(), next: self.next.clone() }
    }
}*/

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node { elem: value, next: self.head.take() });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map( |boxed_node| {
            let node = *boxed_node;
            self.head = node.next;
            node.elem
        })
    }
    pub fn swap_pairs(&mut self) {
		swap_pairs_beyond_link(&mut self.head, 0);
	}
	pub fn iterate(&self, operation: &Fn(&T)) {
		let mut link = &self.head;
		loop {
			match link {
				None => return,
				Some(ref boxed_node) => {
					operation(&boxed_node.elem);
					link = &boxed_node.next;
				}
			}
		}
	}
}
mod test {
    #[test]
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

fn swap_pairs_beyond_link<T>(link: &mut Option<Box<Node<T>>>, depth: usize) {
    match & link {
        None => return,
        Some(ref boxed_node) => {
            match & boxed_node.next {
                None => return,
                Some(_) => {}
            }
        }
    }
    // We have confirmed we have two nodes to swap
    //let prefix = String::from_utf8(vec![b' '; depth]).expect("valid utf8");
    //println!("{:?}Going to swap nodes after {:?}", prefix, link);
    let mut a = link.take().expect("there is no a"); // Box<Node>
    //println!("{:?}a is {:?}", prefix, a);
    let mut b = a.next.take().expect("lack of b"); // Box<Node>
    //println!("{:?}b is {:?}", prefix, b);
    mem::swap(&mut a.next, &mut b.next);
    //println!("{:?}now a is {:?}", prefix, a);
    //println!("{:?}now b is {:?}", prefix, b);
    //mem::swap(link, &mut b.next);
    swap_pairs_beyond_link(&mut a.next, depth+2);
    b.next = Some(a);
    //println!("{:?}now now a is {:?}", prefix, a);
    //println!("{:?}now now b is {:?}", prefix, b);
    mem::replace(link, Some(b));
}






fn print_i32(val: &i32) {
	println!("{:?}", val);
}

fn print_list(list: &List<i32>) {
	list.iterate(&print_i32);
}

/*Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].*/
/*
fn two_sum_slow(numbers: Vec<i32>, total: i32) -> (usize, usize) {
    for (i, item_i) in numbers.iter().enumerate() {
        for (j, item_j) in numbers[i+1..].iter().enumerate() {
            let thispos = i+j+1;
            if *item_i + *item_j == total {
                return (i, thispos);
            }   
        }   
    }
    panic!("Not found");
}
*/
use std::collections::HashMap;

fn two_sum(numbers: Vec<i32>, total: i32) -> (usize, usize) {
    let positions: HashMap<i32, usize> = numbers.iter().enumerate().map(|(tuple_a, tuple_b)| (*tuple_b, tuple_a)).collect();
    for (i, item_i) in numbers.iter().enumerate() {
        let needed = total - item_i;
        if positions.contains_key(&needed) {
            return (i, positions[&needed]);
        }   
    }
    panic!("Not found");
}


fn main() {
    println!("Hello, world!");
    let nums = vec![2, 7, 11, 15];
    let pair = two_sum(nums, 18);
    println!("Result is {:?},{:?}", pair.0, pair.1);
    let mut l = List::new();
    l.push(5);
    l.push(4);
    l.push(3);
    l.push(2);
    l.push(1);
    print_list(&l);
    l.swap_pairs();
    println!("After swap");
    print_list(&l);
}

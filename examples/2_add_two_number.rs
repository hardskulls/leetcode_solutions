
fn main()
{
    let (mut num1, mut num2) : (i32, i32) = (rand::random(), rand::random());

    let max_number = (i32::MAX / 2) - 10;
    num1 = num1.min(max_number);
    num2 = num2.min(max_number);

    let (l1, l2) = (serialize_number(num1), serialize_number(num2));
    let sum = solution(l1, l2);

    assert_eq!(1999998, deserialize_number(sum));
}

/// link: https://leetcode.com/problems/add-two-numbers/
pub fn solution(l1 : OptNode, l2 : OptNode) -> OptNode 
{
    let (first_list, second_list) = (List { head : l1 }, List { head : l2 });
    let (mut first_list, mut second_list) = (first_list.into_iter(), second_list.into_iter());
    let mut buffer = List::new();

    let mut buf_10 = 0;
    loop 
    {
        match (first_list.next(), second_list.next()) 
        {
            (None, None) =>
                {
                    if buf_10 > 0
                    {
                        let opt_node = serialize_number(buf_10);
                        buffer.add_node(opt_node);
                    }
                    return buffer.head.take();
                }
            (Some(y), None) | (None, Some(y)) => update(y + buf_10, &mut buf_10, &mut buffer),
            (Some(x), Some(y)) => update(x + y + buf_10, &mut buf_10, &mut buffer),
        }
    }
}

fn update(sum : i32, buf_10 : &mut i32, buffer : &mut List)
{
    let single_digit = sum % 10;
    *buf_10 = (sum - single_digit) / 10;

    buffer.add_val(single_digit);
}

pub fn sum_node_vals
(
    a : &OptNode,
    b : &OptNode,
) 
    -> OptNum 
{
    match (a, b) 
    {
        (None, None) => None,
        (Some(l), Some(r)) => Some(l.val + r.val),
        (Some(l), None) => Some(l.val),
        (None, Some(r)) => Some(r.val),
    }
}

pub fn deserialize_number(list : OptNode) -> i32 
{
    let mut number : i64 = 0;
    let mut multiplier : i64 = 1;

    let mut next = list;

    while let Some(node) = next 
    {
        let add = node.val as i64 * multiplier;
        number += add;
        multiplier *= 10;
        next = node.next
    }

    number as i32
}

pub fn serialize_number(num : i32) -> OptNode 
{
    let num : i64 = num as i64;
    let mut divider : i64 = 10;
    let mut digit : i64 = num % divider;

    let mut list = ListNode::new(digit as _);

    while num % divider != num
    {
        divider *= 10;
        digit = (num % divider - digit) / (divider / 10);

        list.add(ListNode::new(digit as _));
    }

    Box::new(list).into()
}

pub type OptNode = Option<Box<ListNode>>;
pub type OptNum = Option<i32>;

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode 
{
    pub val : i32,
    pub next : OptNode,
}

impl ListNode 
{
    #[inline]
    pub fn new(val : i32) -> Self 
    { ListNode { next : None, val } }

    #[inline]
    pub fn add(&mut self, new_node : ListNode) 
    {
        let mut next = &mut self.next;
        while let Some(node) = next 
        { next = &mut node.next }
        *next = Some(Box::new(new_node))
    }
}

#[derive(Debug, Default)]
pub struct List 
{ pub head : OptNode, }

impl List {
    #[inline]
    pub fn new() -> Self 
    { Self::default() }

    #[inline]
    fn push_val(&mut self, val : i32) 
    {
        let next = self.head.take();
        let new_node = ListNode { val, next };
        let new_node = Box::new(new_node);

        self.head = Some(new_node);
    }

    #[inline]
    fn add_val(&mut self, val : i32) 
    {
        let new_node = ListNode { val, next : None };
        let mut next = &mut self.head;
        while let Some(node) = next 
        { next = &mut node.next }
        *next = Some(Box::new(new_node))
    }

    #[inline]
    fn add_node(&mut self, node : OptNode) 
    {
        let mut next = &mut self.head;
        while let Some(node) = next 
        { next = &mut node.next }
        *next = node
    }

    #[inline]
    fn pop(&mut self) -> Option<i32>
    {
        self.head
            .take()
            .map(|node| { self.head = node.next; node.val })
    }

    pub fn peek(&self) -> Option<&i32>
    {
        self.head
            .as_ref()
            .map(|node| &node.val) 
    }

    pub fn peek_mut(&mut self) -> Option<&mut i32> 
    {
        self.head
            .as_mut()
            .map(|node| &mut node.val)
    }
}

impl Drop for List 
{
    fn drop(&mut self) 
    {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link 
        {
            cur_link = boxed_node.next.take()
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None
            // so no unbounded recursion occurs.
        }
    }
}

pub mod implementing_iterators 
{
    use super::*;

    impl List {
        pub fn iter(&self) -> Iter 
        { Iter { next : self.head.as_deref() } }

        pub fn iter_mut(&mut self) -> IterMut 
        { IterMut { next : self.head.as_deref_mut() } }

        fn into_iter(self) -> IntoIter 
        { IntoIter { list : self } }
    }

    #[derive(Debug)]
    pub struct Iter<'a> 
    { pub next : Option<&'a ListNode>, }

    #[derive(Debug)]
    pub struct IterMut<'a> 
    { pub next : Option<&'a mut ListNode>, }

    #[derive(Debug)]
    pub struct IntoIter 
    { pub list : List, }

    impl<'a> Iterator for Iter<'a> 
    {
        type Item = &'a i32;

        fn next(&mut self) -> Option<Self::Item> 
        {
            self.next
                .map(|node| { self.next = node.next.as_deref(); &node.val })
        }
    }

    impl<'a> Iterator for IterMut<'a> 
    {
        type Item = &'a mut i32;

        fn next(&mut self) -> Option<Self::Item> 
        {
            self.next
                .take()
                .map(|node| { self.next = node.next.as_deref_mut(); &mut node.val })
        }
    }

    impl Iterator for IntoIter 
    {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> 
        { self.list.pop() }
    }

    impl<'a> IntoIterator for &'a List 
    {
        type Item = &'a i32;
        type IntoIter = Iter<'a>;

        fn into_iter(self) -> Self::IntoIter 
        { self.iter() }
    }

    impl<'a> IntoIterator for &'a mut List 
    {
        type Item = &'a mut i32;
        type IntoIter = IterMut<'a>;

        fn into_iter(self) -> Self::IntoIter 
        { self.iter_mut() }
    }

    impl IntoIterator for List 
    {
        type Item = i32;
        type IntoIter = IntoIter;

        fn into_iter(self) -> Self::IntoIter 
        { self.into_iter() }
    }
}

#[cfg(test)]
mod tests 
{
    // use {deserialize_number, serialize_number};
    use crate::{deserialize_number, serialize_number, solution, List, ListNode};

    #[test]
    fn iterators_test() 
    {
        let mut list = ListNode::new(0);
        list.add(ListNode::new(9));
        list.add(ListNode::new(5));
        list.add(ListNode::new(9));
        list.add(ListNode::new(7));

        let reference = vec![0, 9, 5, 9, 7];
        let list = List { head : Some(Box::new(list)) };
        list.iter().enumerate().for_each(|(idx, num)| assert_eq!(*num, reference[idx]));

        let mut storage = List::new();
        storage.push_val(1);
        storage.push_val(9);
        storage.push_val(7);
        storage.push_val(9);
        storage.push_val(9);
        storage.push_val(6);
        storage.push_val(2);

        let reference = vec![2, 6, 9, 9, 7, 9, 1];

        for (idx, num) in storage.iter().enumerate() 
        { assert_eq!(*num, reference[idx]) }

        for (idx, num) in storage.iter_mut().enumerate()
        { assert_eq!(*num, reference[idx]) }

        for (idx, num) in storage.into_iter().enumerate()
        { assert_eq!(num, reference[idx]) }
    }

    #[test]
    fn basics() 
    {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push_val(1);
        list.push_val(2);
        list.push_val(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_val(4);
        list.push_val(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() 
    {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push_val(1);
        list.push_val(2);
        list.push_val(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn mutable_ref_to_shared_ref() 
    {
        struct Locked { v : Vec<i32>, }
        
        struct Immut<'a> { locked : &'a Locked, }

        let l1 = Locked { v : vec![] };
        let l2 = Locked { v : vec![] };

        let mut immut = Immut { locked : &l1 };
        let mut_ref = &mut immut;

        mut_ref.locked = &l2;

        assert_eq!(&immut.locked.v, &l2.v);

        fn mutate_inner_shared_ref<'a, 'b, 'c>(immut : &'a mut Immut<'b>, new_v : &'c Locked) 
            where 
                'c : 'b, 'c : 'a, 'b : 'a,
        { immut.locked = new_v }

        let old_l = Locked { v : vec![0, 1] };
        let new_l = Locked { v : vec![5, 6, 7] };

        let mut imm = Immut { locked : &old_l };

        mutate_inner_shared_ref(&mut imm, &new_l);

        let (l, r) = (&imm.locked.v, &new_l.v);

        assert_eq!(l, r);
    }

    #[test]
    fn iter() 
    {
        let mut list = List::new();
        list.push_val(1);
        list.push_val(2);
        list.push_val(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        let mut list = List::new();
        list.add_val(1);
        list.add_val(2);
        list.add_val(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() 
    {
        let mut list = List::new();
        list.push_val(1);
        list.push_val(2);
        list.push_val(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }

    #[test]
    fn num_test() 
    {
        assert_eq!(0, 9 - 9 % 10);
    }

    #[test]
    fn deserialize_test()
    {
        let num = i32::MAX - 1;
        let serialized_num = serialize_number(num);
        assert_eq!(num, deserialize_number(serialized_num));
    }

    #[test]
    fn solution_test() 
    {
        let (mut num1, mut num2) : (i32, i32) = (rand::random(), rand::random());

        let max_number = (i32::MAX / 2) - 10;
        num1 = num1.min(max_number);
        num2 = num2.min(max_number);
        let expect = num1 + num2;

        let (l1, l2) = (serialize_number(num1), serialize_number(num2));
        let sum = solution(l1, l2);
        // dbg!(&sum);

        assert_eq!(expect, deserialize_number(sum));
    }
}



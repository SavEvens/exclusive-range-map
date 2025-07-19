use std::ops::{Index, IndexMut, Range};

enum ExclusiveRangeMapNodeColor{
    RED,
    BLACK
}

struct ExclusiveRangeMapNode<T>{
    range : Range<usize>,
    value : T,
    color : ExclusiveRangeMapNodeColor,
    child_left  : Option<Box<ExclusiveRangeMapNode<T>>>,
    child_right : Option<Box<ExclusiveRangeMapNode<T>>>
}


pub struct ExclusiveRangeInvalidInsertError;

enum ExclusiveRangeMapNodeRotateDirection{
    RIGHT,
    LEFT
}

impl<T> ExclusiveRangeMapNode<T>{

    fn new(range : Range<usize>, value : T) -> ExclusiveRangeMapNode<T>{
        ExclusiveRangeMapNode{
            range: range,
            value: value,
            color: ExclusiveRangeMapNodeColor::BLACK,
            child_left: None,
            child_right: None
        }
    }


}

pub struct ExclusiveRangeMap<T>{
    head : Option<Box<ExclusiveRangeMapNode<T>>>,
    size : usize,
}


impl<T> ExclusiveRangeMap<T>{
    pub fn new() -> ExclusiveRangeMap<T>{
        ExclusiveRangeMap{
            head: None,
            size: 0
        }
    }

    pub fn len(&self) -> usize{
        self.size
    }

    fn place_node(start_node : &mut ExclusiveRangeMapNode<T>, new_node : ExclusiveRangeMapNode<T>) -> Result<(), ExclusiveRangeInvalidInsertError> {
        let selected_node = start_node;
        loop{
            
            let left = selected_node.range.start > new_node.range.end;
            let right = selected_node.range.end < new_node.range.start;

            if left && right{
                return Err(ExclusiveRangeInvalidInsertError);
            }

            if new_node.range.start < selected_node.range.start 
                && new_node.range.end > selected_node.range.start {
                return Err(ExclusiveRangeInvalidInsertError);
            }
            if new_node.range.start < selected_node.range.end
                && new_node.range.end > selected_node.range.end {
                return Err(ExclusiveRangeInvalidInsertError);
            }


            if left{
                match &mut selected_node.child_left{
                    Some(child) => {
        return ExclusiveRangeMap::place_node(child, new_node);
                    },
                    None => {
                        selected_node.child_right = Some(Box::new(new_node));
                        return Ok(())
                    }
                }
            }else{
                match &mut selected_node.child_right{
                    Some(child) => {
                        return ExclusiveRangeMap::place_node(child, new_node);
                    },
                    None => {
                        selected_node.child_right = Some(Box::new(new_node));
                        return Ok(());
                    }
                }
            }
        }

    }

    fn balance_tree(&mut self) -> () {
        //TODO
    }

    pub fn insert(&mut self, key : Range<usize>, value : T)
        -> Result<(), ExclusiveRangeInvalidInsertError>{
        match &mut self.head{
            Some(head_node) => {
                let res = ExclusiveRangeMap::place_node(head_node,
                    ExclusiveRangeMapNode::new(key, value));
                match res{
                    Ok(_) => {
                        self.balance_tree();
                        self.size += 1;
                    }
                    Err(_) => {}
                }
                return res;
            },
            None=> {
                self.head = Some(Box::new(ExclusiveRangeMapNode::new(key, value)));
                self.size += 1;
                return Ok(())
            }
        }
    }

    fn find_node(&self, key : &usize) -> Option<&Box<ExclusiveRangeMapNode<T>>> {
        match & self.head{
            Some(_) => {},
            None => {
                return None;
            }
        }
        let mut selected_node : &Box<ExclusiveRangeMapNode<T>>;
        selected_node = self.head.as_ref()?;

        loop{
            let left = selected_node.range.start > *key;
            let right = selected_node.range.end <= *key;
            if left{
                match selected_node.child_left{
                    Some(_) => {
                        selected_node = selected_node.child_left.as_ref()?;
                        continue;
                    },
                    None => {
                        return None;
                    }
                }
            }
            if right{
                match selected_node.child_right{
                    Some(_) => {
                        selected_node = selected_node.child_right.as_ref()?;
                        continue;
                    },
                    None => {
                        return None;
                    }
                }
            }

            return Some(selected_node);
        }

        
    }

    pub fn find(&self, key : &usize) -> Option<&T> {
        match self.find_node(&key) {
            Some(node) => {
                Some(&node.value)
            },
            None => {
                None
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_create_rangepoint_map(){
        let new_map = ExclusiveRangeMap::<u64>::new();

        assert_eq!(new_map.size, 0);
    }

    #[test]
    fn test_basic_rangepoint_map_ops(){
        let mut map = ExclusiveRangeMap::<u64>::new();

        let res1 = map.insert(0..5 as usize, 12);
        let res2 = map.insert(6..12 as usize, 64);
        let res3 = map.insert(14..55 as usize, 1);

        match res1{
            Ok(()) => {},
            Err(_) => {panic!("This insert should pass!");}
        }
        
        match res2{
            Ok(()) => {},
            Err(_) => {panic!("This insert should pass!");}
        }

        match res3{
            Ok(()) => {},
            Err(_) => {panic!("This insert should pass!");}
        }


        assert_eq!(map.size, 3);

        let val1 = map.find(&(2 as usize));
        let val2 = map.find(&(7 as usize));
        let val3 = map.find(&(44 as usize));

        assert_eq!(val1, Some(&12u64));
        assert_eq!(val2, Some(&64u64));
        assert_eq!(val3, Some(&1u64));

        let err1 = map.find(&(5 as usize));
        let err2 = map.find(&(12 as usize));
        let err3 = map.find(&(13 as usize));
        let err4 = map.find(&(55 as usize));

        assert_eq!(err1, None);
        assert_eq!(err2, None);
        assert_eq!(err3, None);
        assert_eq!(err4, None);
    }
    
    #[test]
    fn test_invalid_rangepoint_map_insert(){

        let mut map = ExclusiveRangeMap::<u64>::new();

        let res1 = map.insert(0..20 as usize, 44);
        let res2 = map.insert(20..50 as usize, 55);
        let res3 = map.insert(45..60 as usize, 77);

        match res1{
            Ok(()) => {},
            Err(ExclusiveRangeInvalidInsertError) => {
                panic!("There should be no error in this insertion.");
            }
        }

        match res2{
            Ok(()) => {},
            Err(ExclusiveRangeInvalidInsertError) => {
                panic!("There should be no error in this insertion.");
            }
        }

        match res3{
            Ok(()) => {
                panic!("This insertion should cause an error!");
            },
            Err(ExclusiveRangeInvalidInsertError) => {}
        }
    }
}

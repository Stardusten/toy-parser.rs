use std::{rc::Rc, cell::RefCell, collections::BTreeMap, ops::Deref, borrow::Borrow};

use bimap::BiBTreeMap;

/// 并查集，朴素实现，没有做任何优化
pub struct DisjointSet<T> {
    /// 映射表，将每个 element 映射一个 usize id
    elements: BiBTreeMap<T, usize>,
    /// 直接父亲数组，第 i 个值为 id 为 i 的元素的直接父亲
    fathers: Vec<usize>,
}

impl<T> DisjointSet<T>
    where T: Ord {
    /// 创建一个新的，空的并查集
    fn new() -> Self {
        DisjointSet {
            elements: BiBTreeMap::new(),
            fathers: Vec::new(),
        }
    }

    /// 添加一个新元素，单独成一类
    fn add_element(&mut self, element: impl Into<T>) {
        let id = self.elements.len();
        self.elements.insert(element.into(), id);
        self.fathers.push(id);
    }

    /// 添加一个新元素，与 `class_element` 同一类
    fn add_element_to(&mut self, element: impl Into<T>, class_element: impl Borrow<T>) {
        let id = self.elements.len();
        self.elements.insert(element.into(), id);
        let class_element_id = self.get_id(class_element).unwrap();
        self.fathers.push(*class_element_id);
    }

    /// 添加 elements 中所有元素，单独成一类
    fn add_elements<I>(&mut self, elements: I)
        where I: IntoIterator<Item = T> {
            let mut iter = elements.into_iter();
            if let Some(first_element) = iter.by_ref().next() { // 先取一个元素
                let class_id = self.elements.len(); // 为其开辟一个新类
                self.elements.insert(first_element, class_id);
                self.fathers.push(class_id);
                iter.for_each(|element| { // 迭代器中其他元素和第一个元素属于同一类
                    let id = self.elements.len();
                    self.elements.insert(element, id);
                    self.fathers.push(class_id);
                })
            }
    }

    /// 返回指定 id 对应的元素
    fn get_element(&self, id: &usize) -> Option<&T> {
        self.elements.get_by_right(id)
    }

    /// 返回指定元素对应的 id
    fn get_id(&self, element: impl Borrow<T>) -> Option<&usize> {
        self.elements.get_by_left(element.borrow())
    }

    /// 返回指定元素的父亲
    fn get_father(&self, element: impl Borrow<T>) -> Option<&T> {
        self.get_id(element)
            .and_then(|element_id| self.get_father_by_id(*element_id))
            .and_then(|father_id| self.get_element(&father_id))
    }

    /// 返回指定 id 所表示的元素的父亲
    fn get_father_by_id(&self, id: usize) -> Option<usize> {
        self.fathers.get(id)
            .and_then(|straight_father_id| {
                if id == *straight_father_id {
                    Some(id)
                } else {
                    self.get_father_by_id(*straight_father_id)
                }
            })
    }

    /// 合并两个 id 所表示的元素 (使之拥有相同的父亲)
    fn join_by_id(&mut self, id1: usize, id2: usize) {
        if let (Some(f1), Some(f2)) = 
               (self.get_father_by_id(id1), self.get_father_by_id(id2)) {
            if f1 != f2 { unsafe {
                *(self.fathers.get_unchecked_mut(f1)) = f2;
            }}
        }
    }

    /// 合并两个元素 (使之拥有相同的父亲)
    fn join(&mut self, element1: impl Borrow<T>, element2: impl Borrow<T>) {
        if let (Some(id1), Some(id2)) =
               (self.get_id(element1), self.get_id(element2)) {
            self.join_by_id(*id1, *id2);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::DisjointSet;

    #[test]
    fn test() {
        let mut disjoint_set = DisjointSet::new();
        disjoint_set.add_element("x");
        disjoint_set.add_element("y");
        disjoint_set.add_element_to("z", "x");
        disjoint_set.join("x", "y");
        assert_eq!(disjoint_set.get_father("x"), Some(&"y"));
        assert_eq!(disjoint_set.get_father("y"), Some(&"y"));
        assert_eq!(disjoint_set.get_father("z"), Some(&"y"));
    }

    #[test]
    fn test2() {
        
    }
}
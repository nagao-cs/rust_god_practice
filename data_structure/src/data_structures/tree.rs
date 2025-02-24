pub struct Tree<T> {
    label: T,
    left_tree: Option<Box<Tree<T>>>,
    right_tree: Option<Box<Tree<T>>>,
}

impl<T: std::cmp::PartialEq + std::cmp::PartialOrd> Tree<T> {
    pub fn new(label: T) -> Tree<T> {
        Tree {
            label,
            left_tree: None,
            right_tree: None,
        }
    }

    pub fn insert(&mut self, label: T) {
        if self.label > label {
            if let Some(left) = self.left_tree.as_mut() {
                left.insert(label);
            } else {
                self.left_tree = Some(Box::new(Tree::new(label)));
            }
        } else if self.label < label {
            if let Some(right) = self.right_tree.as_mut() {
                right.insert(label);
            } else {
                self.right_tree = Some(Box::new(Tree::new(label)));
            }
        }
    }

    pub fn search(&self, label: T) -> bool {
        if self.label == label {
            return true;
        }
        if self.label > label {
            if let Some(left) = self.left_tree.as_ref() {
                return left.search(label);
            }
        } else {
            if let Some(right) = self.right_tree.as_ref() {
                return right.search(label);
            }
        }
        false
    }

    pub fn get_label(&self) -> &T {
        &self.label
    }

    pub fn get_left(&self) -> Option<&Tree<T>> {
        self.left_tree.as_deref()
    }

    pub fn get_right(&self) -> Option<&Tree<T>> {
        self.right_tree.as_deref()
    }
}

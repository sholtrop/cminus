pub mod logging {
    use env_logger;
    pub fn init_logger() {
        env_logger::builder().format_timestamp(None).init();
    }
}

pub mod tree {
    use core::fmt;
    use ptree::TreeBuilder;
    pub enum ChildPosition {
        Left,
        Right,
    }
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct TreeIndex(usize);

    pub trait TreeItem {
        /// Get the Left or Right child.
        fn get_child(&self, pos: &ChildPosition) -> Option<TreeIndex>;

        /// Set your own Left or Right child index, return whatever the old [TreeIndex] for that child was.
        fn set_child(&mut self, pos: &ChildPosition, child: TreeIndex) -> Option<TreeIndex>;
    }

    #[derive(Default)]
    pub struct ArenaTree<T: TreeItem + Clone + fmt::Display> {
        arena: Vec<Option<T>>,
    }

    impl<T: TreeItem + Clone + fmt::Display> ArenaTree<T> {
        pub fn new(init: T) -> Self {
            Self {
                arena: vec![Some(init)],
            }
        }

        pub fn get_root_mut(&mut self) -> Option<&mut T> {
            self.arena.first_mut()?.as_mut()
        }

        pub fn get_node_mut(&mut self, index: TreeIndex) -> Option<&mut T> {
            self.arena.get_mut(index.0)?.as_mut()
        }

        pub fn get_node(&self, index: TreeIndex) -> Option<&T> {
            self.arena.get(index.0)?.as_ref()
        }

        /// Set a new `child` node on the `parent` at the chosen `position`. Returns the index of the newly
        /// inserted tree item.
        pub fn set_node(&mut self, parent: &mut T, position: ChildPosition, child: T) -> TreeIndex {
            if let Some(idx) = parent.get_child(&position) {
                self.arena[idx.0] = Some(child);
                idx
            } else {
                let index = TreeIndex(self.arena.len());
                self.arena.push(Some(child));
                parent.set_child(&position, index);
                index
            }
        }

        fn stringify_tree(&self, node: T, builder: &mut TreeBuilder) -> Result<(), &str> {
            let left = node.get_child(&ChildPosition::Left);
            if let Some(child) = left {
                let node = self
                    .get_node(child)
                    .expect("Tree integrity damaged: No left child")
                    .clone();

                builder.begin_child(node.to_string());
                self.stringify_tree(node, builder)?;
                builder.end_child();
            } else {
                builder.add_empty_child("None".to_owned());
            }

            let right = node.get_child(&ChildPosition::Right);
            if let Some(child) = right {
                let node = self
                    .get_node(child)
                    .expect("Tree integrity damaged: No right child")
                    .clone();

                builder.begin_child(node.to_string());
                self.stringify_tree(node, builder)?;
                builder.end_child();
            } else {
                builder.add_empty_child("None".to_owned());
            }
            Ok(())
        }
    }

    impl<T: TreeItem + fmt::Display + Clone> fmt::Display for ArenaTree<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let root = self.arena[0].clone().ok_or(fmt::Error)?;
            let mut builder = TreeBuilder::new(root.to_string());
            self.stringify_tree(root, &mut builder).map_err(|e| {
                write!(f, "Error formatting tree {}", e).unwrap();
                fmt::Error
            })?;
            write!(f, "{}", builder.build().text)
        }
    }
}

use super::iter::{InOrderIter, MidOrderIter, PostOrderIter, PreOrderIter};
use super::node::BinTreeNode;

impl<'a, Tree, T: BinTreeNode<'a, Tree>> BinTreeNodeExt<'a, Tree> for T {}

pub trait BinTreeNodeExt<'a, Tree>: BinTreeNode<'a, Tree> {
    /// 创建一个层序遍历迭代器.
    fn in_order_iter(&self) -> InOrderIter<Self, Tree>
    where
        Self: Sized + Clone,
    {
        let root = if !self.is_empty_subtree() {
            Some(self.clone())
        } else {
            None
        };
        InOrderIter::new(root)
    }

    /// 创建一个前序遍历迭代器.
    fn pre_order_iter(&self) -> PreOrderIter<Self, Tree>
    where
        Self: Sized + Clone,
    {
        let root = if !self.is_empty_subtree() {
            Some(self.clone())
        } else {
            None
        };
        PreOrderIter::new(root)
    }

    /// 创建一个中序遍历迭代器.
    fn mid_order_iter(&'a self) -> MidOrderIter<Self, Tree>
    where
        Self: Sized + Clone,
    {
        MidOrderIter::new(self.clone())
    }

    /// 创建一个后序遍历迭代器.
    fn post_order_iter(&'a self) -> PostOrderIter<Self, Tree>
    where
        Self: Sized + Clone,
    {
        PostOrderIter::new(self.clone())
    }
}

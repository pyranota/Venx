use core::fmt::Debug;

use super::layer::Layer;

impl Debug for Layer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // if self.entries[1] == 0 {
        //     return write!(f, "Empty layer \n");
        // }
        write!(f, " \n ----------- \n Layer \n ----------- \n")?;
        // write!(f, " Entries: {:?} \n ----------- \n", self.entries)?;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.flag == 0 {
                write!(f, " {i} Branch: {:?} \n", node.children)?;
            } else if node.flag == -2 {
                write!(f, " {i} Leaf \n")?;
            } else if node.flag == -1 {
                write!(f, " {i} Free -> {} \n", node.children[0])?;
            } else if node.flag == -3 {
                write!(f, " {i} Fork:   {:?}  \n", node.children)?;
            } else {
                write!(f, " {i} Linked Fork: {:?}  \n", node.children)?;
            }
        }
        write!(f, "\n ----------- \n \n")
    }
}

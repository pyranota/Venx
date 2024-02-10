use core::fmt::Debug;

use super::layer::Layer;

impl Debug for Layer<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // if self.entries[1] == 0 {
        //     return write!(f, "Empty layer \n");
        // }
        write!(f, " \n ----------- \n Layer \n ----------- \n")?;
        write!(f, " Entries: {:?} \n ----------- \n", self.entries)?;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.flag == 0 {
                write!(f, " {i} Branch: {:?} \n", node.children)?;
            }
            if node.flag == 3 {
                write!(f, " {i} Leaf \n")?;
            }
            if node.flag == 9 {
                write!(f, " {i} Reserved \n")?;
            }
            // if node.flag == -1 {
            //     write!(f, ".")?;
            // }
        }
        write!(f, "\n ----------- \n \n")
    }
}

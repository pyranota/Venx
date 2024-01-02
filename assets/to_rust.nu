def main [ ] {
 let list =  open blocks.csv |
  skip 4 |
   get minecraft:acacia_button | 
  enumerate | 
  each { |it| $"\"($it.item)\" => ($it.index + 25)\n" };

  let module = $"pub fn match_block\(name: &str\) -> u32 {
    match name {
      ($list | into string )
      _ => 9999
    }
  }"
  $module | save -f ../crates/venx_core/src/plat/minecraft_blocks.rs
}

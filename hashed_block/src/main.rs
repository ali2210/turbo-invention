// create new module name block_node which provide blockchian like functionalities at basic level. 
// Blockchain is very complex topic. However Pub because this crate accessible in main module
pub mod block_node{
    
    // create Block data struct
    pub struct Block{
        pub id : String,
        pub name : Vec<String>,
        pub tx : i64,
    }

    // importing packages that require for this module
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash,Hasher};
    
    // Impl allow to Block functionalities which is only available for Block structure.
    impl Block{

        pub fn create( self) -> u64{
            
            // Default_Hasher allow us to use hash :: crate 
            let mut hasher = DefaultHasher::new();
            
            // User can't provide data then no block created
            if self.name.get(0) == Some(&" ".to_string()){
               return 0
            }

            // If block contains user name then take hash and return rust return hash (u64) but in future. I will conversion for string too, 
            Hash::hash_slice(&self.name, &mut hasher);
            let block_hash : u64 = hasher.finish();
             return block_hash
        }
    }
}

use crate::block_node::Block;
fn main() {
    
    let mut name : Vec<String> = Vec::new();
    name.push("Ali".to_string());
    let id = "0".to_string();
    let block = Block{id , name, tx: 0};
    let data : u64 = block.create();
    println!(" block create : ({})", data );
    
}


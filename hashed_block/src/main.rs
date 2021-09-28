

// create new module name block_node which provide blockchian like functionalities at basic level. 
// Blockchain is very complex topic. However Pub because this crate accessible in main module
pub mod block_node{
    #[derive(Debug)]
    // create Block data struct
    pub struct Block{
        pub id : String,
        pub name : Vec<String>,
        pub tx : i64,
    }

    // more custom type block
    #[derive(Debug)]
    pub enum Results{
        OK(Block,u64),
        Error(Block)
    }

    // importing packages that require for this module
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash,Hasher};
    
    // Impl allow to Block functionalities which is only available for Block structure.
    impl Block{

        pub fn create( self) -> Results{
            
            // Default_Hasher allow us to use hash :: crate 
            let mut hasher = DefaultHasher::new();
            
            // block created only when user provided his name
            if self.name.get(0) == Some(&" ".to_string()){
                // in this case block boardcast block information
                let result:Results = Results::Error(self);
                return result
             }

            //  build hash of block::Name 
             Hash::hash_slice(&self.name, &mut hasher);
             let block_hash : u64 = hasher.finish();
             
            //  Block boardcast block information along with hash of Block::Name Field
             let result = Results::OK(self, block_hash);
             return result
        }
    }
}

// import crate
use crate::block_node::Block;
use crate::block_node::Results;
fn main() {
    
    let mut name : Vec<String> = Vec::new();
    name.push("Ali".to_string());
    let id = "0".to_string();
    let block = Block{id , name, tx: 0};
    let data : Results = block.create();
    println!(" block create : ({:#?})", data );
    
}


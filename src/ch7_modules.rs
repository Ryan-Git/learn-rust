//see project communicator

//Rules of Module Filesystems
//
//Let’s summarize the rules of modules with regard to files:
//
//If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs.
//If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs.


//Privacy Rules
//
//Overall, these are the rules for item visibility:
//
//If an item is public, it can be accessed through any of its parent modules.
//If an item is private, it can be accessed only by the current module and its child modules.

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}
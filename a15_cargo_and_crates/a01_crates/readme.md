the dev profile Cargo uses when you run cargo build, and the release profile Cargo uses when you run cargo build --release. 
The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.

the cargo crate will run default values for the profile if you haven't specified in cargo.toml else 
 
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3 

these are the default level of optimiztion that are applied in the code but if you override this in cargo.toml then it will run this 

--------------------------DOCUMENTATION-------------------------------------
cargo doc is used to create html for the src directory 
cargo doc --open is used to open crate in browser 


#example is used to make a example section in the html 
#panic is used to make a panic section and tell what all reasons make the function panic
#error what sort error the function may throw so that the user can handle those errors
#unsafe is used to tell on what reasons the function is unsafe
....................Adding Comments.......................
/// is used to add comment or document the item next to it 
//! is used to add comment or document to the comments next to it bascially it is used to add comments to the crate or a module as a whole 

---------------------------Public API--------------------------------
Instead, you can re-export items to make a public structure that’s different from your private structure by using pub use. Re-exporting takes a public item in one location and makes it public in another location, as if it were defined in the other location instead.
--------------------------Login Cargo---------------------------------
cargo login is used to logging into cargo account 
This command will inform Cargo of your API token and store it locally in ~/.cargo/credentials.toml.
cargo publish is used to publish the crate 
...............................Yanking A Version............................
you cant unpublish a verion of code in cargo but you can yank a particular version which means stopping the new projects from adding this crate as a dependency while allowing previous version to keep using:-
cargo yank 
suppose the version is 1.3.4
cargo yank --vers 1.3.4
suppose you want to undo the yank then it will be 
cargo yank --vers 1.3.4 --undo 
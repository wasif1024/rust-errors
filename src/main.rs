use std::process::Command;
use std::io::{Error,ErrorKind};
fn error_handling_example(dir:&str){
println!("\n\n");
let mut commands=Command::new("ls");
match commands.current_dir(dir).status(){
    Ok(cmd)=>Some(cmd),
    Err(e)=>match e.kind(){
        ErrorKind::NotFound=>{
println!("File not found");
None
        },
        _=>{
panic!("Error Occured");
        }
    }
};
    println!("\n\n");

}
fn main() {
    error_handling_example("src");
    error_handling_example("lib");
}

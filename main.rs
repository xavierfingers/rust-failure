use std::net::TcpStream;
use std::io;
fn main()  {
let mut input = String::new();
println!("Enter Host: ");
io::stdin() 
.read_line(&mut input)
.unwrap();
let connection = TcpStream::connect("{input}");
let data = connection.unwrap();
println!("{:#?}", data);
}

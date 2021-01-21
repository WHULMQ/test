//extern crate aerospike;

use aerospike::{Client, ClientPolicy,WritePolicy,as_key,as_bin};

fn main(){
  let client = Client::new(&ClientPolicy::default(), &"192.168.110.250:3000".to_string()).unwrap();
  let policy = WritePolicy::default();
  //policy.timeout = 50;

  let key = as_key!("test", "myset", "mykey");

  let bin1 = as_bin!("mybin", "myvalue");
  let bin2 = as_bin!("mybin2", 12);


  match client.put(&policy, &key, &vec![&bin1,&bin2]) {
      Ok(()) => println!("Record written"),
      Err(err) => println!("Error writing record: {}", err),
  }match client.delete(&WritePolicy::default(), &key) {
    Ok(true) => println!("Record deleted"),
    Ok(false) => println!("Record did not exist"),
    Err(err) => println!("Error deleting record: {}", err),
}
  client.close().unwrap();
}
























// extern crate geohash;
// use std::error::Error;
// use geohash::{encode, decode,neighbor,Direction,Coordinate};
// fn main()->Result<(), Box<Error>> {

//     let s1=String::from("llsllsl,lsllls,jddnvjn,ikd,jjjj");
//     let spl=s1.split(",");
//     for s in spl { 
//       let ss=&s.to_string()[0..2];
//       println!("{}", ss);
//   } 
//     let c=Coordinate{x:103.949759f64,y:30.64109f64};
//     println!("encoding:{}",encode(c,12usize)?);
//     let (c,_,_)=decode("wm3ym6jckcnj")?;
//     println!("decoding ww8p1r4t8 to: {}, {}", c.y, c.x);
//     let sw = neighbor("ww8p1r4t8", Direction::SW)?;
//     println!("neighbor={}", sw);
//     Ok(())
// }
/*extern crate geohash;
use std::error::Error;
use geohash::{encode, decode, neighbor, Direction, Coordinate};
fn main() -> Result<(), Box<Error>> {
  // encode a coordinate
  let c = Coordinate { x: 112.558386f64, y: 37.832386f64 };
  println!("encoding 37.8324, 112.5584: {}", encode(c, 9usize)?);
  // decode a geohash
  let (c, _, _) = decode("ww8p1r4t8")?;
  println!("decoding ww8p1r4t8 to: {}, {}", c.y, c.x);

  // find a neighboring hash
  let sw = neighbor("ww8p1r4t8", Direction::SW)?;

  Ok(())
}*/

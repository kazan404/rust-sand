mod vars;
mod calc;

fn main() 
{
    println!("Hello, world!");
    vars::run();
    let i1 : i32 = 3;
    let i2 : i64 = 3;
    let i3 = 3;

    println!("Address i1 : {:p}", &i1);
    println!("Address i2 : {:p}", &i2);
    println!("Address i3 : {:p}", &i3);

}

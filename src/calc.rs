pub fn Add<T>(a: T, b: T) -> T
{
    a + b
}

pub fn Sub<T>(base: T, away: T) -> T
{
    base - away
}

pub fn Mul<T>(a: T, b: T) -> T
{
    a * b
}

pub fn Div<T>(base: T, div: T) -> T
{
    match div 
    {
        Ok(div) => div,
        Err(err) => println!("Divided Zero!")
    }
    base / biv
}
#[allow(dead_code)]
fn info<T>(item: &T)
where
    T: std::fmt::Display,
{
    println!("{}", item)
}

#[allow(dead_code)]
#[derive()]
struct Foo {
    a: i32,
    b: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_string_slice() {
        let s = "this is the string";
        info(&s)
    }

    #[test]
    fn print_string() {
        let s = String::from("this is the string");
        info(&s);
        let s = "another String".to_string();
        info(&s);
    }

    /* #[test]
    fn print_foo() {
        let foo = Foo {
            a: 2,
            b: String::from("sdr"),
        };
        info(&foo)
    } */
}

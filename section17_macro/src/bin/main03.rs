pub mod route {
    #[macro_export]
    macro_rules! route {
        ( $path:expr, [$( $method:expr ), *] ) => {
            println!("{}", $path);
            $(println!("{}", $method=="Get"));*
        };
    }
}
fn main() {
    route!("/hello", ["GET", "GET"]);
}

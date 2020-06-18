pub mod user;
use user::search_zip::search;
fn main() {
    let addr = search("12289456");
    println!("{}", addr.data());
}

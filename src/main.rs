mod resource;

fn main() {
    println!("Hello, world from The Alderman!");
    let beer = resource::Resource{id: String::from("TheAldreman.beer"), price: 40};
    println!("{}", beer);
}

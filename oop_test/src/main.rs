use oop_test::AveragedCollection;

fn main() {
    let mut collection = AveragedCollection::new();

    collection.add(1);
    collection.add(2);
    collection.add(3);

    println!("average: {}", collection.average());
}

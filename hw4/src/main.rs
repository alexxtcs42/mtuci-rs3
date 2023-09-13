#[derive(Debug)]
struct MyVec<T> {
    vec: Vec<T>
}

impl<T: Clone> MyVec<T> {
    fn new() -> MyVec<T>{
        MyVec { vec: Vec::new() }
    }
    
    fn with_capacity(capacity: usize) -> MyVec<T>{
        MyVec { vec: Vec::with_capacity(capacity) }
    }

    fn push(&mut self, value: T) {
        self.vec.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
     
     fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.vec.len() {
            None
        } else {
            Some(self.vec.remove(index))
        }
     }

     fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
     }

     fn resize(&mut self, new_size: usize, value: T) {
        self.vec.resize(new_size, value)
     }

}
fn main() {
    let mut vec1: MyVec<i32> = MyVec::new();

    vec1.push(9);
    vec1.push(8);
    vec1.push(7);
    vec1.push(6);
    vec1.push(5);
    vec1.push(4);
    vec1.push(3);
    vec1.push(2);
    vec1.push(1);
    vec1.push(0);

    println!("{:?}", vec1.pop());
    println!("{:?}", vec1.remove(1));
    println!("{:?}", vec1.get(1)); // Some(7)
    println!("{:?}", vec1.get(9));  // None

    let mut vec2: MyVec<i32> = MyVec::with_capacity(13);
    println!("{:?}", vec2);
    vec2.resize(10, 5);
    println!("{:?}", vec2);
    vec2.resize(13, 7);
    println!("{:?}", vec2);
}

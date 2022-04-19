
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) ->u32 {
        return self.height * self.width;
    }
} 

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let heart_eyed_cat = '😻';
}

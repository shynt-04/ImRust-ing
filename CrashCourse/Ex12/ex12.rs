// Topic: impl

#[derive(Debug)]
#[warn(dead_code)]

enum Color {
    Red,
    Blue,
    Yellow,
}

struct Dimensions {
    weight : f64,
    height : f64,
    depth: f64,
}

impl Dimensions {

    fn show_dimensions(&self) {
        println!("Dimensions' weight is {:?}", self.weight);
        println!("Dimensions' height is {:?}", self.height);
        println!("Dimensions' depth is {:?}", self.depth);
    }
}

struct Box {
    color : Color,
    weight : f64,
    dimensions: Dimensions,
}

impl Box {
    fn new(color:Color, weight:f64, dimensions:Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    fn show(&self) {
        println!("The box's color is {:?}", self.color);
        println!("The box's weight is {:?}", self.weight);
        self.dimensions.show_dimensions();
    }
}

fn main() {
    let my_dimensions = Dimensions {
        weight: 1.1,
        height: 1.11,
        depth: 11.1,
    };
    let my_box = Box::new(Color::Red, 1.01, my_dimensions);
    my_box.show();

}
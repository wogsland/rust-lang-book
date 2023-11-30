fn main() {
    {
        let width = 30;
        let height = 50;
        println!(
            "Rectangle area is {} pixels squared.",
            area(width, height)
        );
    }

    {
        let rect = (30, 55);
        println!(
            "Rectangle area is {} pixels squared.",
            rect_area(rect)
        );
    }

    {
        let rect = Rectangle{
            width: 25,
            height: 50,
        };
        println!(
            "{:?} area is {} pixels squared.",
            rect,
            rectangle_area(&rect)
        );

        let scale = 30;
        dbg!(scale);
    }



    {
        let rect = Rectangle{
            width: 25,
            height: 25,
        };
        println!(
            "Rectangle area is {} pixels squared.",
            rect.area()
        );

        if rect.width() {
            println!(
                "There be width to this rectangle: {}",
                rect.width
            );
        }

        let lil_rect = Rectangle{
            width: 1,
            height: 2,
        };

        if rect.can_hold(&lil_rect) {
            println!("{:?} can hold {:?}", rect, lil_rect);
        }
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn rect_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }
}

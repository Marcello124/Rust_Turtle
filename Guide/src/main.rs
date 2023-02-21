use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed(18);
  
    
    // introduction


    // example
    // turtle.forward(100.0);
    // for _ in 0..360 {
    //     turtle.forward(3.0);
    //     turtle.right(1.0);
    // }

    // exercise 1
    // turtle.forward(100.0);
    // turtle.right(90.0);
    // turtle.forward(100.0);

    // exercise 2
    // turtle.forward(100.0);
    // turtle.right(270.0);
    // turtle.forward(150.0);

    // exercise 3
    // turtle.right(180.0);
    // turtle.forward(100.0);
    // turtle.right(-90.0); // goes left

    // exercise 4
    // turtle.forward(125.0);
    // turtle.forward(125.0);

    // exercise 5
    // for _ in 0..4 {
    //     turtle.forward(-100.0);
    //     turtle.right(-90.0);
    //     turtle.forward(100.0);
    // }


    // Basics


    // Drawing a triangle
    // for _ in 0..3 {
    //     turtle.forward(100.0);
    //     turtle.right(120.0);
    // }

    // Drawing 3 triangles 
    // turtle.left(30.0);
    // for _ in 0..3 {
    //     for _ in 0..3 {
    //         turtle.forward(100.0);
    //         turtle.right(120.0);
    //     }
    //     turtle.right(120.0);
    // }
    

    // exercise 1
    // turtle.right(30.0);
    // for _ in 0..3 {
    //     for _ in 0..3 {
    //         turtle.forward(100.0);
    //         turtle.right(120.0);
    //     }
    //     turtle.right(120.0);
    // }
    

    // exercise 2
    // let scale = 2.0;
    // turtle.left(30.0);
    // for _ in 0..3 {
    //     for _ in 0..3 {
    //         turtle.forward(100.0 * scale);
    //         turtle.right(120.0);
    //     }
    //     turtle.right(120.0);
    // }

    // exercise 3 
    // for _ in 0..3 {
    //     turtle.left(15.0);
    //     turtle.forward(100.0);
    //     turtle.right(180.0 - 75.0);
    //     turtle.forward(51.76381);
    //     turtle.right(180.0 - 75.0);
    //     turtle.forward(100.0);
    //     turtle.right(180.0 - 30.0);
    //     turtle.right(15.0 + 120.0);
    // }

    // exercise 4
    turtle.set_speed("instant");
    let sides = 360;
    let angle_count = f64::from(sides);
    let angle = 180.0 - 180.0 * (angle_count - 2.0) / angle_count;

    turtle.left(90.0 - angle / 2.0);
    for _ in 0..3 {
        for _ in 0..sides {
            turtle.forward(300.0 / angle_count);
            turtle.right(angle);
        }
        turtle.right(120.0);
    }
}

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
    // turtle.set_speed("instant");
    // let sides = 360;
    // let angle_count = f64::from(sides);
    // let angle = 180.0 - 180.0 * (angle_count - 2.0) / angle_count;

    // turtle.left(90.0 - angle / 2.0);
    // for _ in 0..3 {
    //     for _ in 0..sides {
    //         turtle.forward(300.0 / angle_count);
    //         turtle.right(angle);
    //     }
    //     turtle.right(120.0);
    // }


    // Square


    // example
    // for _ in 0..4 {
    //     turtle.forward(200.0);
    //     turtle.right(90.0);
    // }

    // exercise 1
    // for _ in 0..4 {
    //     turtle.forward(100.0);
    //     turtle.right(90.0);
    // }

    // exercise 2
    // for _ in 0..4 {
    //     turtle.forward(100.0);
    //     turtle.right(45.0);
    // }

    // exercise 3
    // let mut distance = 150.0;
    // for _ in 0..10 {
    //     for _ in 0..4 {
    //         turtle.forward(distance);
    //         turtle.right(90.0);
    //     }
    //     distance -= 10.0;
    // }

    // exercise 4
    // turtle.left(30.0);
    // for _ in 0..4 {
    //     turtle.forward(100.0);
    //     turtle.right(90.0);
    // }

    // exercise 5
    // for _ in 0..5 {
    //     for _ in 0..4 {
    //         turtle.forward(100.0);
    //         turtle.right(90.0);
    //     } 
    //     turtle.right(72.0);
    // }

    // exercise 6
    // turtle.set_speed(24);
    // let square_count = 50;
    // for _ in 0..square_count {
    //     for _ in 0..4 {
    //         turtle.forward(100.0);
    //         turtle.right(90.0);
    //     } 
    //     turtle.right(360.0 / f64::from(square_count));
    // }

    // exercise 7
    // turtle.set_speed("instant");
    // let square_count = 50;
    // let mut distance = 250.0;
    // for _ in 0..=(2 * square_count) {
    //     for _ in 0..4 {
    //         turtle.forward(distance);
    //         turtle.right(90.0);
    //     } 
    //     distance -= distance / (1.0 * f64::from(square_count));
    //     turtle.right(360.0 / f64::from(square_count));
    // }

    // turtle.set_speed("instant");
    // let square_count = 50;
    // let mut distance = 100.0;
    // let mut angle = 360.0 / f64::from(square_count); // home resets heading so I need to store it
    // for _ in 0..=(1 * square_count) {
    //     for _ in 0..4 {
    //         turtle.forward(distance);
    //         turtle.right(angle / 4.0);
    //     } 
    //     turtle.pen_up();
    //     turtle.home();
    //     turtle.pen_down();
    //     distance -= distance / (1.0 * f64::from(square_count));
    //     turtle.right(angle);
    //     angle += 360.0 / f64::from(square_count)
    // }


    // Circle


    // example
    // // for distance 1 and angle 1 (regular 360-gon) radius is approximately 28.65
    // // radius is proportional to distance (side length)
    // for _ in 0..360 {
    //     turtle.forward(1.0);
    //     turtle.right(1.0);
    // }

    // exercise 1
    // for _ in 0..360 {
    //     turtle.forward(1.0);
    //     turtle.left(1.0);
    // }

    // exercise 2
    // for _ in 0..360 {
    //     turtle.forward(3.0);
    //     turtle.left(1.0);
    // }

    // exercise 4 (3 is missing on the site ¯\_(ツ)_/¯)
    // for _ in 0..360 {
    //     turtle.forward(3.0);
    //     // if I double the angle, radius is approximately 2 times smaller
    //     // it's becouse for small enough angles sin(x) = x
    //     // circle is drawn 2 times
    //     turtle.left(2.0);       
    // }

    // exercise 5
    // for _ in 0..360 {
    //     turtle.backward(3.0);
    //     turtle.right(1.0);
    // }

    // exercise 6
    // for _ in 0..360 {
    //     turtle.forward(3.0);
    //     turtle.right(1.0);
    // }
    // for _ in 0..360 {
    //     turtle.forward(3.0);
    //     turtle.left(1.0);
    // }

    // exercise 7
    // for _ in 0..180 {
    //     turtle.forward(3.0);
    //     turtle.right(1.0);
    // }
    // turtle.pen_up();
    // turtle.home();
    // turtle.pen_down();
    // turtle.left(180.0);
    // for _ in 0..180 {
    //     turtle.forward(3.0);
    //     turtle.right(1.0);
    // }

    // exercise 8
    // turtle.set_speed("instant");
    // let count = 20;
    // let mut angle = 0.0;
    // for _ in 0..count {
    //     turtle.right(angle);
    //     for _ in 0..180 {
    //         turtle.forward(2.0);
    //         turtle.right(1.0);
    //     }
    //     angle += 360.0 / f64::from(count);
    //     turtle.pen_up();
    //     turtle.home();
    //     turtle.pen_down();
    // }

    // exercise 9
    // turtle.set_speed("instant");
    // let count = 10;
    // for _ in 0..count {
    //     for _ in 0..360 {
    //         turtle.forward(1.5);
    //         turtle.right(1.0);
    //     }
    //     turtle.right(360.0 / f64::from(count));
    // }
    // turtle.set_pen_size(3.0);
    // turtle.backward(300.0);

    // exercise 10
    // let count = 10;
    // for i in 0..360 {
    //     if i % (360 / count) == 0 {
    //         turtle.pen_up();
    //     } else if i % (360 / count) == 180 / count {
    //         turtle.pen_down();
    //     } 
    //     turtle.forward(3.0);
    //     turtle.right(1.0);
    // }

    // exercise 11
    let sides = 10;
    let distance = 1000.0 / f64::from(sides);
    let angle = 360.0 / f64::from(sides);
    for _ in 0..sides {
        turtle.set_fill_color("orange");
        turtle.begin_fill();
        for _ in 0..3 {
            turtle.forward(distance);
            turtle.left(180.0 - 60.0);
        }
        turtle.end_fill();
        turtle.forward(distance);
        turtle.right(angle);    
    }
    turtle.set_pen_size(3.0);
    turtle.set_fill_color("yellow");
    turtle.begin_fill();
    turtle.left(angle / 2.0);
    for _ in 0..360 {
        turtle.forward(distance / 35.4112 * (f64::from(sides) / 10.0));
        turtle.right(1.0);   
    }
    turtle.end_fill();
}
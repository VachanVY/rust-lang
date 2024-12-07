struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}


impl Rectangle{
    fn area(&self) -> u32{
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32{
        return 2*(self.width + self.height);
    }
}

impl Rectangle{
    fn square(size: u32) -> Rectangle{
        return Rectangle{
            width: size,
            height: size
        };
    }
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

fn main() {
    let mut _user1 = User{
        email: String::from("vvy@gmail.com"),
        username: String::from("vvy"),
        active: true,
        sign_in_count: 1
    };

    let user2 = build_user(
        String::from("vvy@gmail.com"),
        String::from("vvy")
    );

    let _user2 = User{
        email: String::from("viv@gmail.com"),
        username: String::from("viv"),
        ..user2 // remaining fields will be copied from user2
    };

    println!("User1: {}, {}, {}, {}", _user1.email, _user1.username, _user1.active, _user1.sign_in_count);  

    // Tuple Structs
    struct Color(i32, i32, i32);
    let yo = Color(0, 0, 0);
    println!("yo: {}, {}, {}", yo.0, yo.1, yo.2);

    // Tuples
    let rect = (30, 40);
    println!("Area of rectangle: {}", area_tuple(rect));

    // Structs
    let rect1 = Rectangle{
        width: 30,
        height: 40
    };
    println!("Area of rectangle: {}", area(&rect1));
    println!("rect1: {:#?}", rect1);
    println!("Area of rectangle: {}", rect1.area());
    println!("Perimeter of rectangle: {}", rect1.perimeter());

    let square = Rectangle::square(20);
    println!("rect2: {:#?}", square);
}

fn area(rect: &Rectangle) -> u32{
    return rect.width * rect.height;
}

fn area_tuple(dims: (u32, u32)) -> u32{
    return dims.0 * dims.1;
}


fn build_user(email:String, username:String) -> User{
    return User{
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

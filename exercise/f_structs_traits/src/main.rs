// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
//  trait Bite...






// 2. Now create a struct named Grapes with a field that tracks how many grapes are left.  If you
// need a hint, look at how it was done for Carrot at the bottom of this file (you should probably
// use a different field, though).
//
// #[derive(Debug)] // include this line right before your struct definition
// struct Grapes...


// 3. Implement Bite for Grapes.  When you bite a Grapes, subtract 1 from how many grapes are left.
// If you need a hint, look at how it was done for Carrot at the bottom of this file.
//
// impl Bite for...




fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    let mut grapes = Grapes { grapes_left: 25 };
    // carrot.bite();
    // println!("I take a bite: {:?}", carrot);

    // grapes.bite();
    // println!("I take one grape: {:?}", grapes)


    

    // Challenge: Uncomment the code below. Create a generic `bunny_nibbles`
    // function that:
    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut carrot);
    bunny_nibbles(&mut grapes);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

#[derive(Debug)]
struct Grapes {
    grapes_left: i32,
}

// a bit more exploration


fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    println!("Bunny nibbles for awhile: {:?}", item.getleftovers())
}

trait Bite {
    fn bite(self: &mut Self);
    fn getleftovers(self: &mut Self) -> f32;
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
    fn getleftovers(self: &mut Self) -> f32 {
        self.percent_left
    }
}   

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.grapes_left -= 1;
    }
    fn getleftovers(self: &mut Self) -> f32 {
        self.grapes_left as f32
    }
}
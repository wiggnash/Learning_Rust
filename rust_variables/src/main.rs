fn main() {
    println!("Hello, world!");
    // let x = 5;
    // println!("The value of X is {}", x);

    // // this will throw an error lets see why
    // /**
    //  * Cannot assign twice to immutable variable x
    //  * This error happens because in RUST by default when we declare a variable with the let keyword , Its immutable
    //  */
    // x = 6;
    // println!("The value of X is {}", x);

    // === The error can be recitified with this code ====
    // let mut x = 5;
    // println!("The value of X is {}", x);
    // x = 6;
    // println!("The value of X is {}", x);

    // ========== SHADOWING ===========
    // defining the variable with the same name
    // let x = 5;
    // println!("The value of X is {}", x);

    // let x = 6;
    // println!("The value of X is {}", x);

    // ========== Inside a differernt scope ( INNER SCOPE )==========

    // let x = 5;
    // println!("The value of X is {}", x);

    // {
    //     let x = x + 1;
    //     println!("The value of X is {}", x);
    // }

    // let x = 6;
    // println!("The value of X is {}", x);

    // ================================================

    // let x = 5;
    // println!("The value of X is {}", x);

    // let x = "Hello World";
    // println!("The value of X is {}", x);

    // ===========================================

    let mut x = 5;
    println!("The value of X is {}", x);

    x = "Hello World";
    println!("The value of X is {}", x);
}

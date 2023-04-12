struct Animal<T, U> {
    name: T,
    animal_type: T,
    weight: U,
}

fn main() {
    /*
    TASK 1
    *******
    Clippy is a tool that allows you to catch mistakes and improve your code. For this
    exercise, run ``` cargo clippy ``` to see the suggestions and correct the errors.
    */
    let x = 1.2331f64;
    let y = 1.2332f64;
    if y != x {
        // clippy will suggest adding in a margin of error
        println!("Success!");
    }

    let mut res = 42;
    let option = Some(12);
    for x in option {
        // clippy will suggest changing this to 'if let Some(x) = option'
        res += x;
    }
    println!("{}", res);

    /*
    TASK 2
    *******
    Go back to the exercises in Module 1 and Module 2 and run cargo fmt
    Note if this changes your formatting.


    TASK 3
    *******
    Generics Task:
    Create a generic struct for an Animal, containing name, animal type, and age. 
    Then, intitialize 3 different animals, and run cargo build to make sure your animals were initialized correctly.

    */
    struct Animal<T, U> {
        name: T,
        animal_type: T,
        age: U,
    }
    
    let a1 = Animal {
        name: "Sylvie",
        animal_type: "cat",
        age: 3,
    };
    let a2 = Animal {
        name: "Ginger",
        animal_type: "hamster",
        age: 1,
    };
    let a3 = Animal {
        name: "Quincy",
        animal_type: "dog",
        age: 12,
    };

}



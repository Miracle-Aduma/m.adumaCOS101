// Declare a struture
struct Employee {
    ceo:String,
    company:String,
    age:u32
}

fn main() {
    //initialize a struture
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundia Pichia"),
        age:51
    };

    // pass emp1 and emp2 to display()
    display(emp1);
    display(emp2);
}

// Fetch values of specific Structure feilds using the operator an dprint it to the console
fn display(emp:Employee){
    println!("Nme is :{} company is {} age is{}",emp.ceo,emp.company,emp.age);
}

struct Laptop {
    company:String,
    amount:u32
}


impl Laptop {
    fn total_cost(&self)->u32{
        self.amount * 3
    }
}

fn main(){
    let hp = Laptop {
        company:String::from("Hewlett-Packard"),
        amount:650000
    };

    let ibm = Laptop {
        company:String::from("International Business Machines"),
        amount:755000
    };

    let tos = Laptop {
        company:String::from("Toshiba"),
        amount:550000
    };

    let del = Laptop {
        company:String::from("Dell"),
        amount:850000
    };


    let cost = hp.total_cost() + ibm.total_cost() + tos.total_cost() + del.total_cost();

    println!("\nHere's your bill for 12 laptops, 3 from {}, {}, {} and {} 😁\n\n{}", hp.company, ibm.company, tos.company, del.company, cost);

    // println!("{}, {}, {}, {}", hp.amount, ibm.amount, tos.amount, del.amount);
}

// fn bill(emp: Laptop)->u32{
//     let cost = hp.totalCost() + ibm.totalCost() + tos.totalCost() + del.totalCost();

//     cost
// }
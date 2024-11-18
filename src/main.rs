use rand::Rng;
use std::io;
use csv;

fn main() {
    let mut rng = rand::thread_rng();

    let mut rdr = csv::Reader::from_path("capitais.csv").unwrap();
    let tamanho = 197; // hardcoded to avoid using rdr
    //let tamanho = rdr.records().count();
    let random_number: u32 = rng.gen_range(1..=tamanho) as u32;
    let mut i = 0;
    for result in rdr.records() {
        if (i + 1) == random_number {
            let record = result.unwrap();
            //println!("{:?}", record);
            let country = record.get(0).unwrap();
            let capital = record.get(1).unwrap();
            println!("What's the capital of {}", country);
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            if answer.trim().to_uppercase() == capital.to_string().trim().to_uppercase() {
                println!("That's correct!");
            }
            else {
                println!("You failed! It was {}", capital);
            }


            break;
        }
        i += 1;
    }
}


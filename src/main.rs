use std::io;
use rand::Rng;

fn main() {
    println!("Myslim si cislo mezi 1 a 9. Zkus ho uhodnout.");
    let mut input_string = String::new();
    let mut rng =  rand::thread_rng();
    let target_number = rng.gen_range(1..10);
    let mut tries = 0;

    while input_string.trim() != "x" {

        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        tries = tries + 1;

        let number_result = input_string.trim().parse::<u32>();
        match number_result {
            Ok(number) => {
                if number == target_number {
                    if tries < 1 {
                        println!("Ty jsi podvadel, toto fakt neni mozny.");
                    } else if tries == 1 {
                        println!("Hele, kristalovy koule jsou zakazany. Jak jinak by jsi vedel, ze to je opravdu {}, nez ze bys do nejake koukal?", number);
                    } else if tries > 9 {
                        println!("Ty jsi ale moula - vic pokusu nez moznosti? To davas stejne cislo vickrat? Ale jo, je to fakt {}.", number);
                    } else if tries > 4 {
                        println!("Ano, je to {}. Ale toto teda trvalo, ze?", number);
                    } else {
                        println!("Ano, myslel jsem si cislo {}. Jsi fakt borec.", number);
                    }

                    println!("Jestli na to mas, zkus to jeste jednou.");
                    std::process::exit(0);
                } else {
                    println!("Jsi naprosto neschopnej. Tvoje cislo {} je uplne mimo. Zkus to znova.", number);
                }
            }
            Err(_error) => {
                println!("Nezadal jsi cislo, ja uz nehraju.");
                std::process::exit(0);
            }
        }
    }
}

use std::env;

fn main() {
    if let Some(arg) = env::args().collect::<Vec<String>>().get(1) {
        if let Ok(value) = arg.parse::<i32>() {
            for (l, m) in [
                ("łatwy", 2),
                ("przeciętny", 0),
                ("problematyczny", -2),
                ("trudny", -5),
                ("bardzo trudny", -8),
                ("cholernie trudny", -11),
                ("fart", -15),
                ("mistrzowski", -20),
                ("arcymistrzowski", -24),
            ]
            .iter()
            {
                println!("{} -> {:?}", l, value + m);
            }
        } else {
            println!("Wprowadź poprawny numer");
        }
    } else {
        println!("Wprowadź wartość parametru bohatera");
    }
}

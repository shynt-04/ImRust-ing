// Topic: Advanced Match

#[derive(Debug)]
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    let my_ticket = vec![
        Ticket::Backstage(22.2, "CrzyKt".to_owned()),
        Ticket::Vip(33.3, "Shnt_".to_owned()),
        Ticket::Standard(11.1),
    ];

    for ticket in my_ticket {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket with price = {:?}, holder {:?}", price, holder)
            },
            Ticket::Vip(price, holder) => {
                println!("Vip ticket with price = {:?}, holder {:?}", price, holder)
            },
            Ticket::Standard(price) => {
                println!("Standard ticket with price = {:?}", price)
            },
        }
    }
}
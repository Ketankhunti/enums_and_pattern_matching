#[derive(Debug)]
enum Message{
    Quit,
    Move{
        x:i32,
        y:i32
    },
    size(u32,u32,u32)
}
impl Message {
    fn some_value(&self) {
        println!("this is impl block for message enum {:#?}",self)
    }
}

enum Coin{
    peny,
    nickle,
    dime,
    quarter(UsStates)
}

#[derive(Debug)]
enum UsStates{
    california,
    new_york,
    los_angeles,
    san_fransisco
}

fn value_in_cents(coin:Coin)->u8{
    match coin{
        Coin::peny => {
            println!("Lucky Peny");
            1
        },
        Coin::nickle => 5,
        Coin::dime => 10,
        Coin::quarter(usstates) => {
            println!("quarter from {:#?}",usstates);
            25
        }
    }
}

fn main() {
   /* let quit = Message::Quit;

    let Move = Message::Move{
        x:5050,
        y:5050,
    };
    let size = Message::size(1,2,3);

    let mpl = Message::some_value(&Message::Quit);

    println!("{:#?}",quit);
    println!("{:#?}",Move);
    println!("{:#?}",size);*/

    let x = 5;
    let y:Option<usize> = None;

    let sum = x + y.unwrap_or(10);

    println!("{:#?}",sum);

    let value = value_in_cents(Coin::quarter(UsStates::los_angeles));
    println!("{}",value);

    let five = Some(5);
    let six = plus_one(five);
    println!("{:#?}",six);
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        Some(i)=>Some(i+1),
        _ =>None
    }
}
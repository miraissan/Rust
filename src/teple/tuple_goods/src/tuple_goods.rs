fn main() {
    let bunana = ("バナナ",300);
    let apple  = ("リンゴ",200);

    let total = bunana.1 + apple.1;
    print_tuple(bunana);
    print_tuple(apple);
    println!("合計{}円です",total);
}

fn print_tuple(item: (&str,i64)){
    println!("{}を{}円で購入",item.0,item.1);
}


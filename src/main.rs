
fn main() {
    let gift_list = ["A partridge in a pear tree","Two turtle doves","Three french hens","Four calling birds",
"Five golden rings","Six geese a laying","Seven swans a singing","Eight maids a milking","Nine ladies dancing",
"Ten lords a leaping","Eleven pipers piping","Twelve drummers drumming"];
    
    let mut index = 0;
    while index <12 {
        if index == 0 {
            println!("On the first day of christmas, my true love gave to me:");
            println!("{}",gift_list[index]);
            index = index +1;
        }
        if index == 1 {
            println!("On the second day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 2 {
            println!("On the third day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 3 {
            println!("On the fourth day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 4 {
            println!("On the fifth day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 5 {
            println!("On the sixth day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 6 {
            println!("On the seventh day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 7 {
            println!("On the eighth day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 8 {
            println!("On the ninth day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 9 {
            println!("On the 10th day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 10 {
            println!("On the 11th day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        if index == 11 {
            println!("On the 12th day of christmas, my true love gave to me:");
            println!("{:?}", &gift_list[0..index+1]);
            index = index +1;
        }
        
    }

}
        

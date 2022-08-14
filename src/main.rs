use std:: {
    io,
};

fn main() {
    // create pi constant, for all intents and purposes there is a constant in the standard library
    // std::f64::consts::PI, which probably has much higher precision. Since the C++ example uses a
    // developer created constant, do so as well here.
    let pi:f64 = 3.14159;
    // Prompt for input, could always add this to my format input function if wanted.
    println!("Enter the radius of the sphere: ");
    // wait for user input, the input of type String will then now be parsed as 64bit float, which
    // in rust lang will be in the form of an Result which can be either Ok(val)
    // (with the value wrapped) or Err(e) (with an error message wrapped). In this case we are
    // setting the default value in case of err as 0 (explicitly defined as a 64bit float)
    let radius:f64 = format_input().parse::<f64>().unwrap_or(0_f64);
    // volume formula, using the inbuilt powf (pow float, returns a 64bit float),
    let _volume:f64 = 4_f64 * pi * (radius.powf(3_f64))/ 3_f64;
    // Print output of both variables to the screen
    println!("The volume of a sphere with the radius {radius} inches is {_volume} cubic inches.")
}
fn format_input() -> String {
    // create mutable variable ret, define it as type String, and have it hold an empty string,
    // no internal buffer has been created, which means this is fast way of creating a string, but
    // also means reallocation when reusing it and going further than its past allocation capacity.
    let mut ret: String = String::new();
    // open stdin, read until newline, and insert the contents into the ret variable. On Error
    // print the line "Failed to read line."
    io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read line");
    // with ret as the source remove any whitespace from the right. The result of this operation is
    // reference to str (&str), convert it to String as this is the variable that function is
    // expected to return.
    // ret.trim_end().to_string()
    // Yet another way of returning a string from a immutable str
    String::from(ret.trim_end())
}
fn main() {
    let temp_fahrenheit = 32.0;
    let temp_celsius = fahernheit_to_celsius(temp_fahrenheit);
    println!("{} fahrenheiht = {} celsius", temp_fahrenheit, temp_celsius)
}

fn fahernheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

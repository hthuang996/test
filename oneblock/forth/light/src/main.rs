enum Light {
    RED(u8),
    YELLOW(u8),
    GREEN(u8),
}

trait Signal {
    fn get_time(light: Light) -> u8;
}

impl Signal for Light {
    fn get_time(light: Light) -> u8 {
        match light {
            Light::RED(x) => return x,
            Light::YELLOW(x) => return x,
            Light::GREEN(x) => return x,
        }
    }
}

fn main() {
    let red = Light::RED(8);
    let yellow = Light::YELLOW(15);
    let green = Light::GREEN(20);
    println!("Time of red light: {}", Light::get_time(red));
    println!("Time of yellow light: {}", Light::get_time(yellow));
    println!("Time of green light: {}", Light::get_time(green));
}

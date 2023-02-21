//I'm writing a weightlifting calculator So for example.. say I
//want 50kg total. I put in 50kgs and the bars weight ( my bar weighs 17.5kg) so it takes away the
//17.5kgs for the bar.. left with 32.5kgs. Then that needs to be spread evenly on each side so it
//   halves it 16.25kgs. So it gives you the number 16.25kgs and maybe even go further in saying
//   exactly what plates. So in the end it would be like "16.25kgs each side.. 10kg plate, 5kg
//   plate, 1.25kg plate"


use std::io;
//A function that gets user input and returns a float
fn get_input() -> f32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f32 = input.trim().parse().expect("Please type a number!");
    return input;
}

//A function that will calculate the amount and type of plates needed on both sides to match the
//target weight
fn calculate(target_weight: f32) {
    let bar_weight = 17.5;
    let mut target_weight = target_weight - bar_weight;
    let mut plates = [25.0, 20.0, 15.0, 10.0, 5.0, 2.5, 1.25];
    let mut plate_count = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut plate_total = 0.0;

    //This loop will calculate the amount of plates needed on each side
    for i in 0..plates.len() {
        while target_weight > 0.0 {
            if target_weight >= plates[i] {
                plate_count[i] += 1.0;
                target_weight -= plates[i];
            } else {
                break;
            }
        }
        plate_total += plate_count[i];
    }

    //This loop will print out the amount of plates needed on each side
    for i in 0..plates.len() {
        if plate_count[i] > 0.0 {
            println!("{}kg plate: {}", plates[i], plate_count[i]);
        }
    }

    //This will print out the total amount of plates needed on each side
    println!("Total amount of plates needed: {}", plate_total);
}

fn main() {
    println!("Please enter the target weight: ");
    let target_weight = get_input();
    println!("The target weight is: {}kg", target_weight);
    calculate(target_weight);
}

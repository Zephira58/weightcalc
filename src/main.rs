use std::io;

struct Plate {
    weight: f64,
    name: &'static str,
}

impl Plate {
    fn new(weight: f64, name: &'static str) -> Plate {
        Plate { weight, name }
    }
}

struct WeightCalculator {
    total_weight: f64,
    bar_weight: f64,
    plates: Vec<Plate>,
}

impl WeightCalculator {
    fn new(total_weight: f64, bar_weight: f64) -> WeightCalculator {
        let plates = vec![
            Plate::new(20.0, "20kg"),
            Plate::new(15.0, "15kg"),
            Plate::new(10.0, "10kg"),
            Plate::new(5.0, "5kg"),
            Plate::new(2.5, "2.5kg"),
            Plate::new(1.25, "1.25kg"),
        ];
        WeightCalculator {
            total_weight,
            bar_weight,
            plates,
        }
    }

    fn calculate_weight_per_side(&self) -> f64 {
        (self.total_weight - self.bar_weight) / 2.0
    }

    fn calculate_plates_needed(&self, weight_per_side: f64) -> Vec<&str> {
        let mut plates_needed: Vec<&str> = Vec::new();
        let mut remaining_weight = weight_per_side;
        for plate in &self.plates {
            while remaining_weight >= plate.weight {
                plates_needed.push(plate.name);
                remaining_weight -= plate.weight;
            }
        }
        plates_needed
    }
}

fn main() {
    // Get the total weight from the user
    println!("Enter the total weight (in kg):");
    let mut total_weight = String::new();
    io::stdin().read_line(&mut total_weight).unwrap();
    let total_weight: f64 = total_weight.trim().parse().unwrap();

    // Get the weight of the bar from the user
    println!("Enter the weight of the bar (in kg):");
    let mut bar_weight = String::new();
    io::stdin().read_line(&mut bar_weight).unwrap();
    let bar_weight: f64 = bar_weight.trim().parse().unwrap();

    // Calculate the weight for each side of the bar
    let calculator = WeightCalculator::new(total_weight, bar_weight);
    let weight_per_side = calculator.calculate_weight_per_side();

    // Determine the plates needed for each side
    let plates_needed = calculator.calculate_plates_needed(weight_per_side);

    // Print the result
    println!("Weight per side: {:.2}kg", weight_per_side);
    println!("Plates needed per side: {:?}", plates_needed);
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
}

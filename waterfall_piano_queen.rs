// Defining a module for this Animal Rescue and Adoption Program
mod AnimalRescueAdoptionProgram {
    // Definition of Struct for Animal's Basic Information
    struct Animal {
        name: String,
        age: u32,
        species: String,
    }

    // Function to Create a Database of Animal's basic information
    fn create_database(input_name: String, input_age: u32, input_species: String) -> Animal {
        Animal {
            name: input_name,
            age: input_age,
            species: input_species,
        }
    }

    // Function to Record Information for an Animal in the Database
    fn record_information(name: String, age: u32, species: String) {
        let animal_details: Animal = create_database(name, age, species);
        println!("The animal's name is {}, age is {} and species is {}", animal_details.name, animal_details.age, animal_details.species);
    }

    // Function to Search for a Particular Animal in the Database
    fn search_database(name: String) -> Option<Animal> {
        let mut animals: Vec<Animal> = Vec::new();
        animals.push( create_database("Lily".to_string(), 3, "Dog".to_string()) );
        animals.push( create_database("Rhea".to_string(), 2, "Cat".to_string()) );
        animals.push( create_database("Max".to_string(), 5, "Parrot".to_string()) );
        animals.push( create_database("Leo".to_string(), 1, "Fish".to_string()) );

        for animal in animals {
            if animal.name == name {
                return Some(animal);
            }
        }
        None
    }

    // Function to Show the List of all Available Animals in the Database
    fn show_list_animals() {
        let mut animals: Vec<Animal> = Vec::new();
        animals.push( create_database("Lily".to_string(), 3, "Dog".to_string()) );
        animals.push( create_database("Rhea".to_string(), 2, "Cat".to_string()) );
        animals.push( create_database("Max".to_string(), 5, "Parrot".to_string()) );
        animals.push( create_database("Leo".to_string(), 1, "Fish".to_string()) );

        println!("List of Available Animals:");
        for animal in animals {
            println!("Name- {}, Age- {}, Species- {}", animal.name, animal.age, animal.species);
        }
    }

    // Function to Promote Animal Welfare and Responsible Pet Ownership
    fn promote_animal_welfare() {
        println!("Encourage pet owners to spay and neuter their pets. This will limit the number of abandoned animals.");
        println!("Educate pet owners about proper care for their pets, such as providing adequate shelter, food, water, and veterinary care.");
        println!("Practice the three Rs - Reduce, Reuse, and Recycle - to create a more sustainable environment for animals and people alike.");
    }
}
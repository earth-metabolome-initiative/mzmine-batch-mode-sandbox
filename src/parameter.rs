use std::collections::HashMap;

// Idea:
    // create a struct to ease the creation of the XML
    // name -> name of batchstep
    // HashMap -> key: name of batchstep's parameter
    //         -> value: Vec of string for all relative values

pub struct Parameter {
    name: String,
    // all parameters are matched with their correspondant value(s) through a hashmap
        //Key -> String -> name of parameter of the batchstep (name of batchstep is stored in name variable of this class)
        //Value -> Vec<String> First entry of the vector is the name of the parameter, all the rest are the value needed to be printed
    parameters: HashMap<String, Vec<String>>
}

impl Parameter {
    pub fn new(name: &str, keys:Vec<&str>, values: Vec<Vec<&str>>) -> Self {
        let mut hashed_parameters = HashMap::new();

        for (i, key) in keys.iter().enumerate() {
            // for each key create an iterator to get the corrispondent vec of parameters
            hashed_parameters.insert(key.to_string(), values[i].iter().map(|&v| v.to_string()).collect());
        }

        Parameter {
            name: name.to_owned(),
            parameters: hashed_parameters,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    /// return &HashMap
    pub fn get_parameters(&self) -> &HashMap<String, Vec<String>> {
        &self.parameters
    }

    // insert a new parameter HashMapped
    pub fn add_parameter(&mut self, key:&str, values: Vec<&str>) {
        self.parameters.insert(key.to_string(), values.into_iter().map(|s| s.to_string()).collect());
    }
}
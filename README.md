# ! This crate is currently under construction !

# mzmine
Mzmine is a Java based tool used for Mass Spectrometry data analysis. For more information please consult the [website](http://mzmine.github.io/) and the [github page](https://github.com/mzmine/mzmine).

## Installation
To install mzmine, please follow the official steps illustrated in the [documentation](https://mzmine.github.io/mzmine_documentation/getting_started.html) and/or by downloading [mzmine portable versions or installers](https://github.com/mzmine/mzmine3/releases/latest).

# mzbatch_generator
Here we present a Rust crate to generate mzbatch file (XML format) used to lauch the batch mode without passing through the GUI.

This crate provides an automatized way to produce .mzbatch files (XML format) to fully launch mzmine4 from the command line in batch mode.<br>
It's important to notice that it *does not* implement all possibile parameters/modules for all batchsteps, but offers an easy way to implement further batchsteps/parameters/modules when needed.

## Create mzbatch file

-> currently working on the final implementation

## run batch mode
```bash
source /etc/profile
mzmine4 -batch /path/to/file.mzbatch 
```

## Add new batchstep/parameter/module
### move to the directory
Move to the desired [batchstep parameters directory](https://github.com/earth-metabolome-initiative/mzmine-batch-mode-sandbox/tree/main/src/batchsteps) and create the realtive parameter rust file:
```bash
cd src/batchsteps/desired_batchstep_parameters
touch parameter_name.rs
```
### Create the struct
Create struct representing the parameter characteristics in order to be able to serialize it and correctly generate the parameter string in the final XML file.<br>
Let's say we want to recreate the following parameter:

```xml
<parameter name="Text in identity" selected="false">a random string</parameter>
```

The relative struct to exploit quick-xml serialization will look like:

```rust
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename = "parameter")]
pub struct Parameter {
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "@selected")]
    selected: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty", rename = "$text")]
    value: String,
}

impl Parameter {
    pub fn new(name: &str, selected: Option<bool>, value: &str) -> Self {
        Parameter {
            name: name.to_owned(),
            selected,
            value: value.to_owned(),
        }
    }

// getters and setters
// like get_name(), get_value(), set_value(), is_selected(), select(), deselect(), ...
// implement as you need or following already existing parameters with similar structures
}
```

### Add new struct to batchstep_parameters.rs, batchsteps.rs and lib.rs
Move back one directory to exit parameters and go to the batchstep_parameters.rs and add a new line with the name of the file in which the new struct is present (without .rs extension):
```rust
pub mod parameter_name;
```
Move back once more and in batchstep.rs, look for the pub mod batchstep_parameters{} desired and inside the curly brackets add the following rust snippet:
```bash
cd ..
```
```rust
pub mod parameter_name;
```
Finally, move back once more and in lib.rs, search for the needed pub mod batchstep_parameters{} and the same code line as before.

### Add the new struct to batchstep/parameter enum
Move to the batchstep/parameter in which the new parameter/module needs to be added, import the struct at the beginning of the file and look for the enum which is used to populate the batchstep/parameter vector, finally add the new variant:
```rust
use serde::{Serialize, Deserialize};

// import struct
use crate::batchsteps::batchstep_parameters::Parameter; <- our new parameter

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename = "batchstep")]
pub struct DesiredBatchstep {
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    #[serde(rename = "parameter")]
    parameters: Vec<PossibleParameters>,
}
...
...
...

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PossibleParameters{
    OldParameter(OldParameter),
    Parameter(Parameter) // <- inserting our new parameter
}
```

Please note that the use of decorators like #[serde(rename = "@name")] are used to serialize the struct via quick-xml (and serde), and is key to for the correct XML element generation. <br>
In brief:
```txt
@ -> Node's attribute
$ -> Node's value
  -> (nothing) Node's name
```
For more info on @/$ or blank, please consult quick-xml's [documentation](https://docs.rs/quick-xml/latest/quick_xml/index.html).<br>
The Parameter is now ready to be serialized in the selected batchstep.

## Add nested parameters
Let's say a new parameter contains a set of nested parameters like:
```xml
<parameter name="File names">
    <file>file1</file>
    <file>file2</file>
</parameter>
```
In order to generate this XML element, we follow the same structure as illustrated before to create the "File names" object, but we need to add a vector of a file type (another struct), here the previous version with updated structure to allow for sub-parameter file:
```rust
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct FileNames {
    #[serde(rename = "@name")]
    name: String,
    
    #[serde(rename = "file")]  // <- decorator to serialize object with file tag
    files: Vec<InputFile>,     // <- Vectof of subparameter struct
}


impl FileNames{
    pub fn new() -> Self {
        FileNames {
            name: "File names".to_owned(),
            files: vec![],
        }
    }

// add getters and setters
    

    pub fn files_length(&self) -> usize{
        self.files.len()
    }

    pub fn add_file_name(&mut self, element: InputFile){
        self.files.push(element);
    }

    pub fn remove_file_name(&mut self, name: &str) {
        self.files.retain(|file| file.get_name() != name);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct InputFile {
    #[serde(rename = "$text")]    // pay attention to decorator's syntax to recreate exactly the XML's element
    name: String
}

impl InputFile{
// implement new(), getters and setters for sub-parameter
}
```

## More on nested structures
Let's imagine file would have it's own sub-parameter, like:
```xml
<parameter name="File names">
    <file>file1</file>
        <subparameter attribute="false"/>
    <file>file2</file>
        <subparameter attribute="false"/>
</parameter>
```
To recreate said structure, it's only needed to add a vector of type "subparameter" to file struct with the needed decorators, like:
```rust
...
#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct InputFile {
    #[serde(rename = "$text")]    // pay attention to decorator's syntax to recreate exactly the XML's element
    name: String

    #[serde(rename = "subparameter")]
    subparameters = Vec<SubParameter>
}

impl InputFile{
// implement new(), getters and setters for sub-parameter
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct SubParameter {
    #[serde(rename = "@attribute")]
    attribute: bool
}
```


---
Repository initiated with [mapp-metabolomics-unit/mapp-repo-copier-template](https://github.com/mapp-metabolomics-unit/mapp-repo-copier-template).

This project was created by MAPP Metabolomics Unit.

Contact: pierre-marie.allard@unifr.ch

# ! This crate is currently under construction !

# Mzmine
Mzmine is a Java based tool used for Mass Spectrometry data analysis. For more information please consult the [website](http://mzmine.github.io/) and the [github page](https://github.com/mzmine/mzmine).

## Installation
To install Mzmine, please follow the official steps as requested in the [documentation](https://mzmine.github.io/mzmine_documentation/getting_started.html) by downloading [mzmine portable versions or installers](https://github.com/mzmine/mzmine3/releases/latest).

# mzbatch_generator
Here we present a Rust crate to generate mzbatch file (XML format) used to lauch the batch mode without passing through the GUI.

This crate provides an automatized way to produce .mzbatch files (XML format) to fully launch mzmine4 from the command line in batch mode.
It's important to notice that it *does not* implement all possibile parameters/modules for all batchsteps, but offers an easy way to implement further batchsteps/parameters/modules when needed.

## Create mzbatch file

-> currently working on the final implementation

## run batch mode
```bash
source /etc/profile
mzmine4 -batch /path/to/file.mzbatch 
```

## Add new batchstep/parameter/module
Move to the desired [batchstep parameters directory](https://github.com/earth-metabolome-initiative/mzmine-batch-mode-sandbox/tree/main/src/batchsteps) and create the realtive parameter rust file:
```bash
cd src/batchsteps/desired_batchstep_parameters
touch parameter_name.rs
```

Create struct representing the parameter characteristics in order to be able to serialize it and correctly generate the parameter string in the final XML file.
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn is_selected(&self) -> &Option<bool> {
        &self.selected
    }

    pub fn select(&mut self) {
        self.selected = Some(true);
    }

    pub fn deselect(&mut self) {
        self.selected = Some(false);
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_owned();
    }
}
```

- **Github repository**: <https://github.com/earth-metabolome-initiative/mzmine-batch-mode-sandbox/>
- **Documentation**: <https://earth-metabolome-initiative.github.io/mzmine-batch-mode-sandbox/>

## Where to start

Have a look at the [docs folder](https://github.com/earth-metabolome-initiative/mzmine-batch-mode-sandbox/docs), this is very likely where your files are.

---
Repository initiated with [mapp-metabolomics-unit/mapp-repo-copier-template](https://github.com/mapp-metabolomics-unit/mapp-repo-copier-template).

This project was created by MAPP Metabolomics Unit.

Contact: pierre-marie.allard@unifr.ch

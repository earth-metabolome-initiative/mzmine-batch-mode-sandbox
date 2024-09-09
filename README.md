# ! This crate is currently under construction 

# Mzmine
Mzmine is a Java based tool used for .mzml file analysis originated from Mass Spectrometry
## mzbatch-generator
This crate provides an automatized way to produce .mzbatch files (XML format) to fully launch mzmine4 from the command line in batch mode. </ br>
It *does not* implement all possibile parameters/modules for all batchsteps, but offers an easy way to implement further options when needed and implement mzmine's batchmode in more complex environments.

## Installation
To install Mzmine, please follow the official steps as requested in the [documentation](https://mzmine.github.io/mzmine_documentation/getting_started.html) by downloading [mzmine portable versions or installers](https://github.com/mzmine/mzmine3/releases/latest).

# mzbatch-generator
Here we present a Rust crate to generate mzbatch file (XML format) used to lauch the batch mode without passing through the GUI.

## Create mzbatch file

-> currently working on the final implementation

# run batch mode
```bash
source /etc/profile
mzmine4 -batch /path/to/file.mzbatch 
```


## Add new batchstep/parameter/module
If not already present, the implementation of a new batchstep/parameter/module should be pretty straight forward.

'''rust
'''



- **Github repository**: <https://github.com/earth-metabolome-initiative/mzmine-batch-mode-sandbox/>
- **Documentation**: <https://earth-metabolome-initiative.github.io/mzmine-batch-mode-sandbox/>

## Where to start

Have a look at the [docs folder](https://github.com/earth-metabolome-initiative/mzmine-batch-mode-sandbox/docs), this is very likely where your files are.

---
Repository initiated with [mapp-metabolomics-unit/mapp-repo-copier-template](https://github.com/mapp-metabolomics-unit/mapp-repo-copier-template).

This project was created by MAPP Metabolomics Unit.

Contact: pierre-marie.allard@unifr.ch

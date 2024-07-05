use std::fs::File;
use std::io::Result;
use std::fs;

use std::env;

use simple_xml_builder::XMLElement;

mod batchsteps;
use batchsteps::FileList;

fn parse_args(args: Vec<String>) -> Vec<Vec<String>> {
    let mut parsed_args: Vec<Vec<String>> = Vec::new();
    let mut tmp_arg: Vec<String> = Vec::new();

    for arg in args {
        if arg.contains('-') {
            if !tmp_arg.is_empty() {
                parsed_args.push(tmp_arg.clone());
            }
            tmp_arg.clear();
        }
        tmp_arg.push(arg);
    }
    
    if !tmp_arg.is_empty() {
        parsed_args.push(tmp_arg);
    }

    parsed_args.split_off(1) // do not return file path
}

fn get_input_files(flags: Vec<(usize, String)>, parsed_args: Vec<Vec<String>>) -> Vec<String>{
    let i_index = flags.iter().position(|(_, v)| v == "-i").unwrap();

    let mut tmp_file_list: Vec<String> = Vec::new();

    let paths = fs::read_dir(parsed_args[i_index][1].clone()).unwrap();

    for path in paths {
        tmp_file_list.push(path.unwrap().path().display().to_string());
    }

    tmp_file_list
}


fn main() -> Result<()> {


    if let Ok(current_dir) = env::current_dir() {
        println!("Current working directory: {}", current_dir.display());
    } else {
        eprintln!("Failed to get current working directory");
    }




    // Get all the parameters inputted, each input is represented as a vector with the command in first place 
    // and other parameters as following elements
    let parsed_args: Vec<Vec<String>> = parse_args(env::args().collect());

    // Check that a minimal set of inputs is present
    // For now minimal requirements are:
    // - -o output XML file name a
    // - -i input file list
    
    //let flags: Vec<String> = parsed_args.iter()
    //    .filter_map(|inner_vec| inner_vec.first().cloned())
    //    .collect();

    let mut flags: Vec<(usize, String)> = parsed_args.iter().enumerate().map(|(i, v)| (i, v[0].clone())).collect();

    assert!(flags.iter().any(|(_, s)| s == "-i" || s == "--input"), "No -i / --input provided");
    assert!(flags.iter().any(|(_, s)| s == "-o" || s == "--output"), "No -o / --output provided");


    // Create XML file based on user parameters
    let o_index = flags.iter().position(|(_, v)| v == "-o").unwrap();
    //println!("{:?}", parsed_args[index]);

    let mut file = File::create(parsed_args[o_index][1].clone())?;


    let mut batch = XMLElement::new("batch");
    batch.add_attribute("mzmine_version", "4.0.1");



    let mut file_list = FileList::new(get_input_files(flags.clone(), parsed_args.clone())).gen_XML_element();
    //println!("{:?}", file_list.files);
    //println!("{}", file_list.number_of_files);

    batch.add_child(file_list);




    // Possible match cases:
    // - -i /--input -> for inputting the file list
    // - -md / --massdetection 
    // - -cb // --chromatogrambuilder 
    // - -s // --smoothing 
    // - -d // --deconvolution 
    // - -iso // --isotopegrouper
    // - -rf // --rowsfilter
    // - -e // --export -> takes parameters gnps or sirius

    //for arg[1] in parse_args {
    //    match input {
    //        // "example" => println!("Matched 'example'"),
    //        // "test" => println!("Matched 'test'"),
    //        // _ => println!("No match"),
    //        
    //        "-i" => createBatchstep
    //
    //
    //    }
    //}

    batch.write(&mut file)?;

    Ok(())
}

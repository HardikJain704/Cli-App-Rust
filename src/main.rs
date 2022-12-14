use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{
    fs,
    io::{BufReader},
};
use structopt;
use structopt::StructOpt;

extern crate json_value_remove;

#[derive(StructOpt, Debug)]
enum Git {
    #[structopt(name = "add")]
    Add {
        // #[structopt(short = "i")]
        // id: u32,
        #[structopt(short = "n")]
        name: String,
        #[structopt(short = "a")]
        address: String,
    },
    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(long = "id")]
        id: u32,
        // name: String,
    },
    #[structopt(name = "delete")]
    Delete {
        #[structopt(short = "m")]
        id: u32,
    },
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Students {
    id: u32,
    name: String,
    address: String,
}

fn save(name: String, address: String) -> Result<(), Box<dyn std::error::Error>> {
    let s = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        //  .truncate(true)
        .open("example.json")
        .unwrap();
    let data = fs::read_to_string("example.json").unwrap();
    let mut datas: Vec<Students> = Vec::new();
    if !data.is_empty() {
        datas = serde_json::from_str(&data).unwrap();
    }
    //  let mut writer = BufWriter::new(file);
    let a: u32 = datas.len().try_into().unwrap();

    let student = Students {
        id: a + 1,
        name: name,
        address: address,
    };
    // a+= 1;

    datas.push(student);

    serde_json::to_writer(&s, &datas)?;
    Ok(())
}

fn main() {
    match Git::from_args() {
        Git::Add { name, address } => {
            // println!("{:?} {:?} {:?}", id, name, address);
            // if action == "Add" {
            match save(name, address) {
                Ok(_) => println!("saved successfully"),
                Err(_) => println!("error"),
            }
        }
        Git::Delete { id } => {
            let file = File::open("example.json").unwrap();
            let reader = BufReader::new(file.try_clone().unwrap());
            let mut data1: Vec<Students> = serde_json::from_reader(reader).unwrap();
            for (i, s) in data1.clone().iter().enumerate() {
                if s.id == id {
                    data1.remove(i);
                    // println!("{:?}", "data removed");
                    fs::write(
                        "example.json",
                        serde_json::to_string(&data1).unwrap().as_bytes(),
                    );
                    println!("{:?}", "data removed");
                    return;
                } 
            }
            println!("{:?}", "data not there");
        }
        Git::Fetch { id } => {
            let file = File::open("example.json").unwrap();
            let reader = BufReader::new(file);
            let data1: Vec<Students> = serde_json::from_reader(reader).unwrap();
            for i in data1 {
                if i.id == id {
                    println!("{:?}", i);
                     return;
                }
               
            }
            println!("{}", "student not found");
        }
    }
}

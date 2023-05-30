extern crate csv;
use lazy_static::lazy_static;
use std::collections::HashMap;

use std::fs::{OpenOptions, self};
use std::io::{prelude::*, BufReader};
use std::path::{PathBuf, Path};
use walkdir::{DirEntry, WalkDir};
use walkdir::Result;

lazy_static! {
    static ref HM: HashMap<&'static str, u8> = {
        let m = HashMap::from([
            ("txt", 1),
            ("add_gene.txt", 2),
            ("add_gene.2.txt", 3),
            ("c.txt", 4),
            ("c.add_gene.txt", 5),
            ("c.add_gene.2.txt", 6),
            ("c.r.txt", 7),
            ("c.r.add_gene.txt", 8),
            ("c.r.add_gene.2.txt", 9),
        ]);
        m
    };
    static ref MH: HashMap<u8, &'static str> = {
        let m = HashMap::from([
            (1, "txt"),
            (2, "add_gene.txt"),
            (3, "add_gene.2.txt"),
            (4, "c.txt"),
            (5, "c.add_gene.txt"),
            (6, "c.add_gene.2.txt"),
            (7, "c.r.txt"),
            (8, "c.r.add_gene.txt"),
            (9, "c.r.add_gene.2.txt"),
        ]);
        m
    };
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn to_csv(input: PathBuf, output: PathBuf) -> Result<()> {
    let mut wtr = csv::Writer::from_path(output).unwrap();
    let read_file = OpenOptions::new()
        .read(true)
        .open(&input)
        .expect("read_file fail");

    let mut index = 0;
    let lines_iter = BufReader::new(read_file).lines().skip(3);
    wtr.write_record(&["T1", "T2", "T3", "T4", "TS", "GS", "SEQ", "Annotation"])
        .expect("header fail");
    for line in lines_iter {
        index += 1;
        let mut row: Vec<String> = Vec::new();
        let mut count: u8 = 0;
        let line = line.unwrap();

        for part in line.split_whitespace().skip(1) {
            if count < 6 {
                row.push(part.to_string());
            } else if count == 6 {
                row.push(part.to_string());
                row.push(String::new());
            } else {
                row[7] = row.get(7).unwrap().to_string() + " " + part;
            }
            count += 1;
        }
        if row.len() != 8 {
            println!(
                "Unlegal Data: '{}' in {}: {}row",
                &line,
                &input.display(),
                &index
            );
            continue;
        }
        wtr.write_record(row).expect(&line);
    }
    wtr.flush().expect("flush");
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    println!("Start: {}", path);


    for directory in WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .skip(1)
    {
        sort(directory.unwrap().into_path()).unwrap();
    }
}

fn sort(root: PathBuf) -> Result<()> {
    // let root_name = root.file_name().unwrap().to_str().unwrap();
    let root_path = root.as_path();
    //let mut file_map: HashMap<String, Vec<String>> = HashMap::new();

    let mut map:HashMap<String, [u8;3]> = HashMap::new();
    for entry in WalkDir::new(&root)
        .max_depth(1)
        .into_iter()
        .filter_entry(|e| !is_hidden(e)).filter(|e| e.as_ref().unwrap().path().is_file())
    {
        let path = entry?.into_path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_parts: Vec<&str> = file_name.splitn(2, ".").collect();
        let a = file_parts[0].to_owned();
        let b = file_parts[1].to_owned();
        // println!("{} {}",a, b);
        let score = *(HM.get(b.as_str()).unwrap());
        let aa = &a;
        if map.get(aa).is_none() {
            map.insert(aa.to_string(), [0,0,0]);
        }
        match score {
            1|2|3 => {
                let x = map.get_mut(&a).unwrap();
                    if x[0] < score {
                        x[0]=score;
                    }
            },
            4|5|6 => {
                let y = map.get_mut(&a).unwrap();
                    if y[1] < score {
                        y[1]=score;
                    
                }},
            7|8|9 => {
                let z = map.get_mut(&a).unwrap();
                    if z[2] < score {
                        z[2]=score;
                    }
               
            },
            _ => {}
        }

                
        // map.get_mut("chromosome-1-1").unwrap()[0]=9;
        // println!("{:?}",map);
        

        // if let Some(files) = file_map.get_mut(&a) {
        //     files.push(b);
        // } else {
        //     file_map.insert(a, vec![b]);
        // }
    }
    let path_map = map.iter()
    .map(|(k,v)| ( to_csv_path(k, v, root_path)));
    let mut f: Vec<[PathBuf;2]> = Vec::new();
    for v in path_map {
        
        // println!("{:?}", v);
        for [input,output] in v {
            f.push([input,output]);
            // to_csv(input, output).unwrap();
        }
        
    
    }
    for p in f {
        to_csv(p[0].to_path_buf(),p[1].to_path_buf()).unwrap();
    }
    // println!("{:?}", f);


    // for (folder_name, files) in file_map {
    //     let new_path = &root.join(&folder_name);
        
    //     let mut upstream: u8 = 0;
    //     let mut upstream_file = String::new();
    //     let mut complement: u8 = 0;
    //     let mut complement_file = String::new();
    //     let mut r_c: u8 = 0;
    //     let mut r_c_file = String::new();
        

    //     for file in files {
    //         let score = HM.get(file.as_str()).unwrap();
    //         if file.contains("r") {
    //             if score > &mut r_c {
    //                 r_c = *score;
    //                 r_c_file = file;
    //             }
    //         } else if file.contains("c") {
    //             if score > &mut complement {
    //                 complement = *score;
    //                 complement_file = file;
    //             }
    //         } else {
    //             if score > &mut upstream {
    //                 upstream = *score;
    //                 upstream_file = file;
    //             }
    //         }
    //     }

    //     if !fs::metadata(new_path).is_ok() {
    //         fs::create_dir(new_path).unwrap();
    //     }

    //     let first_name: String = format!("{}-{}", root_name, folder_name);

    //     let upstream_file = build_path(format!("{}.{}",&folder_name,&upstream_file), &root);
    //     let complement_file = build_path(format!("{}.{}",&folder_name,&complement_file), &root);
    //     let r_c_file = build_path(format!("{}.{}",&folder_name,&r_c_file), &root);

    //     if upstream != 0 {
    //         to_csv(upstream_file, new_path.join(format!("{}.csv", first_name))).unwrap();
    //     }
    //     if complement != 0 {
    //         to_csv(
    //             complement_file,
    //             new_path.join(format!("{}-c.csv", first_name)),
    //         )
    //         .unwrap();
    //     }
    //     if r_c != 0 {
    //         to_csv(r_c_file, new_path.join(format!("{}-cr.csv", first_name))).unwrap();
    //     }
    // }
    Ok(())
}

// fn build_path(s: String, root_path: &PathBuf) -> PathBuf {
//     let mut path = root_path.clone();
//     path.push(s);
//     path
// }

fn to_csv_path<'a>(front:&'a String, behind: &'a [u8;3],root: &Path) ->Vec<[PathBuf;2]> {
    let mut result = Vec::new();
    let names = ["upstream","complement","r_c"];
    let mut count = 0;
    let root_path = PathBuf::from(root);
    let new_path = root_path.join(front);
    if !fs::metadata().is_ok() {
        fs::create_dir(new_path).unwrap();
    }
    for index in behind{
        if index != &0 {
            let input = root_path.join(format!("{}.{}",front,MH.get(index).unwrap()));
            let o = format!("{}.{}.{}.csv",root.file_name().unwrap().to_str().unwrap(),front,names[count]);
            let output = new_path.join(&o);
            result.push([input,output]);
        }
        count+=1;
    }
    result
}

use std::{env, fs::File, io::Read, error};

fn main() -> Result<(), Box<dyn error::Error>>{
    let args:Vec<String> = env::args().collect();
    assert!(args.len() >= 3);
    let mut lwrcase_idx:usize = 0;
    for (idx, string) in (&args).into_iter().enumerate().skip(2) {
        if string.contains("-idc") { // I don't care about case sentivity 
            lwrcase_idx = idx;
            break;
        }
    }

    let file_name = &args[1];
    let qidx = 2 + (lwrcase_idx > 0) as usize;
    let mut query_string = args[qidx].clone();
    if lwrcase_idx > 0 {
        query_string = query_string.to_lowercase();
    }
    
    let mut file = File::open(file_name)?;
    let mut fbuf:String = String::from("");
    file.read_to_string(&mut fbuf)?;
    if lwrcase_idx > 0 {
        fbuf = fbuf.to_lowercase();
    }
    
    let mut found_lines = String::from("");
    for fline in fbuf.split("\n"){
        if fline.contains(&query_string) {
            println!("File {} contains the word {}", file_name, args[qidx]);
            found_lines.push_str(fline);
            found_lines.push_str("\n");
        }
    }
    println!("Query {} from file {} found in lines:\n\n{}", args[qidx], file_name, found_lines);

    Ok(())
}

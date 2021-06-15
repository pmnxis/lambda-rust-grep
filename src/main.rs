use std::env;
use std::io;
use std::io::prelude::*;

fn heavy_stdinln_vec_str() -> Vec<String> {
    let mut __stdin = io::stdin();
    let mut __stdinlck = __stdin.lock();
    let mut slice: Vec<u8> = Vec::new();
    __stdinlck.read_to_end(&mut slice).expect("error");
    let stuff =  String::from_utf8(slice).unwrap();
    let sliced = stuff.lines().map(|s| s.to_string()).collect();
    sliced
}

fn filtered_line(input: &String, inc: &Option<String>, exc: &Option<String>) -> String {
    let none_str = "".to_string();
    
    let is_included = match inc {
        Some(i) => input.contains(i),
        None => true,
    };
    let should_exclude = match exc {
        Some(x) => Some(input.contains(x)),
        None => None,
    };

    match (is_included, should_exclude){
        (_, Some(true)) => none_str,
        (true, _) => format!("{}\n", input),
        _ => none_str,
    }
}

fn get_specific_parm(args: &Vec<String>, prefix: String) -> Option<String>
{
    let mut ret: Option<String> = None;
    for i in 0.. args.len() - 1
    {
        if args[i] == prefix {
            ret = Some(args[i+1].clone());
        }
    }
    ret
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let inc = get_specific_parm(&args, "-F".to_string());
    let exc = get_specific_parm(&args, "-v".to_string());
    heavy_stdinln_vec_str().iter().for_each(|x| print!("{}", filtered_line(x, &inc, &exc)));
}

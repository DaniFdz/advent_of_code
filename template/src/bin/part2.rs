use {{crate_name}}::part2::process;

fn main(){
    let input = include_str!("../../inputs/input2.txt");
    let output = process(input);
    let _ = !dbg!(output);
} 


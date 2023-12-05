use day_02::*;

fn main(){
    divan::main();
}

#[divan::bench]
fn part1(){
    part1::process(divan::black_box(include_str!(
        "../inputs/input1.txt"
    )));
}


#[divan::bench]
fn part2(){
    part1::process(divan::black_box(include_str!(
        "../inputs/input2.txt"
    )));
}

use convolve2d::DynamicMatrix;
use std::convert::TryInto;
use std::env;
use std::fs::File;

fn to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);
    let fname: &String = &args[1];
    println!("fname: {}", fname);

    // Read png into vector
    let decoder = png::Decoder::new(File::open(fname).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let (width, height) = reader.info().size();
    println!("width: {}, height: {}", width, height);
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    println!("info: {:?}", info);
    // println!("buf: {:?}", buf);
    let img = DynamicMatrix::new(width as usize, height as usize, buf);

}

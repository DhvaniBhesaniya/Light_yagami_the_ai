// use bincode::config::standard;
// use ndarray::{ArrayD, IxDyn};
// use ndarray_npy::NpzReader;
// use std::collections::HashMap;
// use std::fs::File;
// use std::io::BufReader;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let home = dirs::home_dir().ok_or("No home dir")?;
//     let npz_path = home.join(".dependencies/models/voices.npz");
//     let bin_path = home.join(".dependencies/models/voices-v1.0.bin");

//     println!("Reading from {:?}", npz_path);
//     let file = File::open(&npz_path)?;
//     let mut npz = NpzReader::new(file)?;

//     let mut voices: HashMap<String, Vec<Vec<Vec<f32>>>> = HashMap::new();

//     let names = npz.names()?;
//     println!("Found {} names", names.len());
//     // println!("Names: {:?}", names); // Too verbose if many

//     for name in names {
//         println!("Processing voice: {}", name);
//         let voice_name = name.trim_end_matches(".npy").to_string();

//         let arr: ArrayD<f32> = match npz.by_name(&name) {
//             Ok(a) => a,
//             Err(e) => {
//                 println!("Failed to read array for {}: {}", name, e);
//                 continue;
//             }
//         };
//         // Expected shape: [510, 1, 256]
//         // Convert to Vec<Vec<Vec<f32>>>

//         // We can use arr.outer_iter() to iterate over the first dimension
//         let mut voice_data = Vec::new();
//         for outer in arr.outer_iter() {
//             let mut inner1 = Vec::new();
//             for inner in outer.outer_iter() {
//                 let mut inner2 = Vec::new();
//                 for val in inner.iter() {
//                     inner2.push(*val);
//                 }
//                 inner1.push(inner2);
//             }
//             voice_data.push(inner1);
//         }

//         voices.insert(voice_name, voice_data);
//     }

//     println!("Writing to {:?}", bin_path);
//     let mut file = File::create(&bin_path)?;
//     // Use bincode to encode
//     let size = bincode::encode_into_std_write(&voices, &mut file, standard())?;
//     println!("Wrote {} bytes", size);

//     Ok(())
// }

fn main() {
    println!("Hello, world!");
}
wit_bindgen_rust::export!("lengthof.wit");

struct Lengthof;
impl lengthof::Lengthof for Lengthof {


    fn length_of(arr: Vec<String>) -> String {
        arr.len().to_string()
    }
}



// struct IntegerArray {
//     data: Vec<i32>,
// }

// impl IntegerArray {
//     fn new(data: Vec<i32>) -> Self {
//         IntegerArray { data }
//     }

//     fn get_length(&self) -> usize {
//         self.data.len()
//     }
// }
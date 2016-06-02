#[macro_use]
extern crate ruru;
use ruru::{AnyObject, Boolean, Class, Fixnum, Hash, VM, Array};
use ruru::types::Argc;
use ruru::traits::Object;

use std::collections::{HashSet, HashMap};
// Really from the examples
#[no_mangle]
pub extern fn initialize_rust_color() {
    methods!(
        Hash, // type of `self` object
        itself, // name of `self` object which will be used in methods

        fn greedy_color() -> Hash {
            let mut colors = HashSet::new();
            let mut colored: HashMap<i64, usize> = HashMap::new();
            let keys = itself.send("keys", vec![]).to::<Array>();
            for x in 0..keys.send("length", vec![]).to::<Fixnum>().to_i64() {
                let key = keys.at(x).to::<Fixnum>();
                let list = itself.at(key).to::<Array>();
                let key = keys.at(x).to::<Fixnum>();
                let mut used_colors = HashSet::new();

                for y in  0..list.send("length", vec![]).to::<Fixnum>().to_i64() {
                    let color_key = keys.at(y).to::<Fixnum>().to_i64();
                    match colored.get(&color_key){
                        Some(color) => {
                            used_colors.insert(color.clone());
                        }
                        _ => {}
                    }
                }
                match colors.difference(&used_colors).cloned().nth(0){
                    None =>{
                        let next_color = colors.len()+1;
                        colored.insert(key.to_i64(), next_color);
                        colors.insert(next_color);
                    }
                    Some(next_color) =>{
                        colored.insert(key.to_i64(), next_color);
                    }
                }
            }
            let mut ruby_hash = Hash::new();
            for (k, v) in colored {
                ruby_hash.store(Fixnum::new(k), Fixnum::new(v as i64));
            }
            ruby_hash
        }
    );

    Class::from_existing("Hash").define(|itself| {
        itself.def("greedy_color", greedy_color);
    });
}

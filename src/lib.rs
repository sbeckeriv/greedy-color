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
            let mut ruby_hash = Hash::new();
            let hash_keys = itself.send("keys", vec![]).to::<Array>();

            for hash_index in 0..hash_keys.send("length", vec![]).to::<Fixnum>().to_i64() {
                let key = hash_keys.at(hash_index).to::<Fixnum>();
                let list = itself.at(key).to::<Array>(); // consumes key?

                let hash_key = hash_keys.at(hash_index);
                let mut used_colors = HashSet::new();

                for list_index in  0..list.send("length", vec![]).to::<Fixnum>().to_i64() {
                    let color_key = list.at(list_index);
                    let color = ruby_hash.at(color_key);
                    if !color.value().is_nil(){
                        let col=color.to::<Fixnum>().to_i64();
                        used_colors.insert(col);
                    }
                }
                match colors.difference(&used_colors).cloned().nth(0){
                    None =>{
                        let next_color = (colors.len()+1) as i64;
                        ruby_hash.store(hash_key, Fixnum::new(next_color));
                        colors.insert(next_color);
                    }
                    Some(next_color) =>{
                        ruby_hash.store(hash_key, Fixnum::new(next_color));
                    }
                }
            }
            ruby_hash
        }
    );

    Class::from_existing("Hash").define(|itself| {
        itself.def("greedy_color", greedy_color);
    });
}

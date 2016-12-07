#[macro_use]
extern crate ruru;
use ruru::{AnyObject, Boolean, Class, Fixnum, Hash, VM, Array};
use ruru::Object;
use ruru::types::Argc;
use std::collections::{HashSet, HashMap};

// Really from the examples
#[no_mangle]
pub extern "C" fn initialize_rust_color() {
    methods!(
        Hash, // type of `self` object
        itself, // name of `self` object which will be used in methods

        fn greedy_color() -> Hash {
            let mut colors = HashSet::new();
            let mut ruby_hash = Hash::new();
            itself.each(|key,list|{
                let mut used_colors = HashSet::new();
                let array_list = list.try_convert_to::<Array>();
                // why the double array here?
                for x in array_list {
                    for color_key in x{
                    let color = ruby_hash.at(color_key);
                    if !color.is_nil(){
                        let col=unsafe{color.to::<Fixnum>().to_i64()};
                        used_colors.insert(col);
                    }
                    }
                }

                match colors.difference(&used_colors).cloned().nth(0){
                    None =>{
                        let next_color = (colors.len()+1) as i64;
                        ruby_hash.store(key, Fixnum::new(next_color));
                        colors.insert(next_color);
                    }
                    Some(next_color) =>{
                        ruby_hash.store(key, Fixnum::new(next_color));
                    }
                }
            });
            ruby_hash
        }
    );

    Class::from_existing("Hash").define(|itself| {
        itself.def("greedy_color", greedy_color);
    });
}

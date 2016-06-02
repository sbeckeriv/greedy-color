require 'rubygems'
require 'bundler/setup'
Bundler.require
require 'fiddle'
library = Fiddle.dlopen('/Users/becker/trash/hash_color/target/release/libhash_color.dylib')


Fiddle::Function.new(library['initialize_rust_color'], [], Fiddle::TYPE_VOIDP).call
x = {1 =>[12,2,3],
2=>[3,12],
12=>[2,1],
3=>[2,1]}
  puts x.greedy_color


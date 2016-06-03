require 'rubygems'
require 'bundler/setup'
Bundler.require
require 'fiddle'
library = Fiddle.dlopen(File.join(File.dirname(__FILE__),'target','release','libhash_color.dylib'))

Fiddle::Function.new(library['initialize_rust_color'], [], Fiddle::TYPE_VOIDP).call
#File.read("/Users/becker/county_adjacency.txt")
require "csv"
require "set"
require "pp"
hash = {}
current = nil
CSV.foreach(File.join(File.dirname(__FILE__),"county_adjacency.txt"), :encoding => "r:ISO-8859-1", :col_sep => "\t"){|row|
  if row[1]
    current = row[1].to_i
  end
  hash[current] ||= Array.new
  hash[current].push(row[3].to_i) unless row[3].to_i == current
}
#pp hash
colored = hash.greedy_color
hash.each{|k,v| puts "#{k}:#{colored[k]} => #{v.map{|x| colored[x]}.inspect},"}

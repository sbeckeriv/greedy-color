require 'rubygems'
require 'bundler/setup'
Bundler.require
require 'fiddle'
require 'benchmark'
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
class Hash
  def greedy
    colors = Set::new
    colored = Hash::new
    self.each{|k,v|
      colors_used = Set::new
      v.map{|c| colors_used.add(colored[c]) if colored[c]}
      first = (colors-colors_used).first
      unless first
        first = colors.size+1
        colors.add(first)
      end
      colored[k]=first
    }
    colored
  end
end
#colored = hash.greedy_color
#hash.each{|k,v| puts "#{k}:#{colored[k]} => #{v.map{|x| colored[x]}.inspect},"}
puts "#{hash.size} nodes"
n = 5000
Benchmark.bmbm do |x|
  x.report("ruby:") { for i in 1..n; hash.greedy ; end }
  x.report("rust:") { for i in 1..n; hash.greedy_color ; end }
end

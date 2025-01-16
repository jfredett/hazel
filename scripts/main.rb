#TODO: Issue when parsing types w/ lots of generics (like Familiar), check on that.
require 'async'
require 'async/barrier'
require 'find'
require 'pry'
require 'tree_sitter'
require 'tree_stand'


require_relative 'api'
require_relative 'field'
require_relative 'location'
require_relative 'query'
require_relative 'scm'
require_relative 'source_tree'
require_relative 'type'
require_relative 'variant'

# TODO: Marshall this and only load if the SHA has changed. -- Make a Marshall class, probably I should name this
# something, damn it I'm getting attached.
SourceTree.load!
Query.load!

barrier = Async::Barrier.new
Async do
  barrier.async do
    STDERR.puts "Parsing Structs..."
    Query[:Struct].run!
  end

  # barrier.async do
  #   STDERR.puts "Parsing Enums..."
  #   Query[:Enum].run!
  # end

  # barrier.async do
  #   STDERR.puts "Parsing Traits..."
  #   Query[:Trait].run!.each do |result|
  #     match = result.matches[0]
  #     name = match["trait.name"]
  #     location = Location.new(
  #       result.path,
  #       name.range.start_point.row,
  #       name.range.start_point.column
  #     )
  #     Type.trait(name.text, location)
  #   end
  # end

  barrier.wait

  STDERR.puts "Finding APIs..."
  Type.types.each do |ty|
    barrier.async do
      STDERR.puts "  #{ty.kind} #{ty.name}"
      ty.find_apis!
    end
  end

  barrier.wait

  puts "@startuml"
  Type.types.each do |ty|
    barrier.async do
      puts ty.to_uml
      puts ""
    end
  end

  barrier.wait

  Type.types.each do |ty|
    barrier.async do
      puts ty.uml_links
      puts ""
    end
  end
  puts "@enduml"

  barrier.wait
end

STDERR.puts "Done!"

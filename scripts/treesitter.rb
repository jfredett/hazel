require 'tree_sitter'
require 'pry'
require 'tree_stand'
require 'find'


class Location
  attr_accessor :file, :line, :column
end

class Type 
  attr_accessor :name, :location, :kind

  def self.register!(name, type)
    @registry ||= {}
    @registry[name] = type
  end

  def self.struct(name, definition)
    new(:struct, name, definition).tap { |o| register!(name, o) }
  end

  def self.enum(name, defintiion)
    new(:enum, name, defintion).tap { |o| register!(name, o) }
  end

  def self.trait(name, defintiion)
    new(:trait, name, defintion).tap { |o| register!(name, o) }
  end

end


def Impl
  attr_accessor :type, :location, :trait

  def initialize(type, location, trait = nil)
    @type = type; @location = location; @trait = trait
  end

  def implements_trait?
    !@trait.nil?
  end
end

def Query
  def self.load!
    Find.find('queries') do |path|
      # find all the `.scm`, subdirs, and .rbs in `queries`, load each into a class that allows it to be executed across
      # the whole source tree, producing the model as a result.
    end
  end
end


class SourceTree
  def self.load!
    Find.fine('src') do |path
      # port the stuff from below, load it all once, and then have an API for running query across the whole tree
    end
  end

end

TreeStand.configure do
  config.parser_path = '.parsers'
end

parser = TreeStand::Parser.new('rust')

# Load queries to a hash
queries = {
  structs_enums_traits: File.read('queries/structs_enums_traits.scm'),
  # These should be parameterized?
  impl_for: File.read('queries/impl_for.scm'), # Searches for a bare impl block for a given typename.
  impl_for_trait: File.read('queries/impl_for_trait.scm'), # Given a type name and a trait, searches for an impl block for that type for that trait.
  trait_def: File.read('queries/trait_def.scm'), # Given a trait name, searches for the trait definition.
  struct_def: File.read('queries/struct_def.scm'), # Given a struct name, searches for the struct definition.
}

source_files = Find.find('src') do |path|
  if File.directory?(path)
    Find.prune if File.basename(path).start_with?('.')
  else
    next unless path.end_with?('.rs')
    source_code = File.read(path)
    tree = parser.parse_string(source_code)
    matches = tree.query(queries[:structs_enums_traits])
    matches.each do |match|
      key = if match.has_key?("struct.name")
        "struct.name"
      elsif match.has_key?("enum.name")
        "enum.name"
      elsif match.has_key?("trait.name")
        "trait.name"
      end

      obj = match[key]

      puts "#{path}:#{obj.range.start_point.row}: #{key.gsub(".name","")} #{obj.text}"
    end
  end
end

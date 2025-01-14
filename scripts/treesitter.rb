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


class Impl
  attr_accessor :type, :location, :trait

  def initialize(type, location, trait = nil)
    @type = type; @location = location; @trait = trait
  end

  def implements_trait?
    !@trait.nil?
  end
end

class Query
  attr_reader :src_path, :code

  # Find all the queries in the queries directory, and load them into a registry by name. Queries can be either `.scm`
  # or .`.rb` files. `.scm` files are assumed to be treesitter queries to be executed on the source tree and return
  # their matches from the #run! method.
  #
  # `.rb` files are assumed to be ducks of the Query class (you can subclass it to get most of the behavior for most
  # queries). This will let you implement your own `#run!` method, and you can refer to other queries by their name in
  # the registry a la `Query[:NameOfQuery].run!`.
  #
  # This method loads all these queries into a flat registry, nested directories are allowed, but the namespace is flat,
  # So you will need to ensure that your queries have unique names across the whole `queries` directory and it's
  # children.
  def self.load!
    @registry ||= {}
    Find.find('queries') do |path|
      Find.prune if File.directory?(path) && File.basename(path).start_with?('.')
      case
      when path.end_with?('.scm')
        scm = SCM.new(path)
        @registry[scm.key] = scm
      when path.end_with?('.rb')
        require path
        klass_name = Query.key_for(path)
        klass = Object.const_get(klass_name)
        @registry[klass_name] = klass.new
      else
        next
      end
    end
  end

  def self.[](key)
    @registry[key]
  end

  # TODO: this should support a filter over the find results. When executing I want to cache the parse if I can, but I
  # could generate a list of paths using another find, this assumes no changes during the generation, but locking is a
  # topic for another day (I'd probably go for an `inotify` or w/e the tool is).
  #
  # The filter would have to run at query time, ideally it would be separate from the query definition, maybe a separate
  # registry? `just treesitter filter_name query_name`?
  #
  # I can extract it from `just` and maybe make the filter dynamic.

  def initialize(path)
    @src = path
    @code = File.read(path)
  end

  def run!
    raise 'abstract'
  end

  def self.key_for(path)
    snake_case = File.basename(path).gsub('.scm', '').gsub('.rb', '')
    # "camel_case" -> :CamelCase
    snake_case.split('_').map(&:capitalize).join.to_sym
  end

  def key
    self.class.key_for(@src)
  end
end

class SCM < Query
  def run!
    puts "Running #{key}"
    SourceTree.query(self.code)
  end
end

class SourceTree
  class << self
    attr_reader :sources

    def load!
      @sources = {}

      Find.find('src') do |path|
        Find.prune if File.directory?(path) && File.basename(path).start_with?('.')
        next unless path.end_with? '.rs'
        @sources[path] = Entry.new(path)
      end
    end

    def query(code)
      self.sources.map do |path, entry|
        puts "Querying #{entry.path}"
        matches = entry.content.query(code)
        if matches.any?
          Result.new(path, entry.content.query(code))
        else
          nil
        end
      end.reject(&:nil?).flatten
    end

    def parse(code)
      self.parser.parse_string(code)
    end

    def parser
      return @parser if @parser

      ::TreeStand.configure do
        config.parser_path = '.parsers'
      end

      @parser = TreeStand::Parser.new('rust')
    end
  end

  class Result
    attr_accessor :path, :matches

    def initialize(path, matches)
      @path = path
      @matches = matches 
    end
  end

  class Entry
    attr_accessor :path

    def initialize(path)
      @path = path
    end

    def content
      puts "Parsing #{@path}"
      @content ||= SourceTree.parse(File.read(@path))
    end
  end

end



SourceTree.load!
Query.load!

results = Query[:Structs].run!
binding.pry
results.each do |result|
  key = "struct.name"
  result.matches.each do |match|
    obj = match[key]
    puts "#{result.path}:#{obj.range.start_point.row}: #{key.gsub(".name","")} #{obj.text}"
  end

rescue => e
  binding.pry
  abort
end

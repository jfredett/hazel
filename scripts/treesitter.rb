require 'tree_sitter'
require 'pry'
require 'tree_stand'
require 'find'


class Location
  attr_accessor :file, :line, :column

  def initialize(file, line, column)
    @file = file; @line = line; @column = column
  end
end

class Type
  attr_accessor :name, :location, :kind
  attr_reader :api

  def initialize(name, location, kind)
    @name = name; @location = location; @kind = kind
  end

  def find_apis!(refresh: false)
    @api = nil if refresh
    return @api unless @api.nil?

    @api = {}

    Query[:Impl].run!(self.name).each do |result|
      result.matches.each do |match|
        name = match["function.name"]
        params = match["function.parameters"]
        return_type = match["function.return_type"]
        trait_name = match["trait.name"]
        trait_args = match["trait.args"]

        trait = if trait_name.nil?
          nil
        elsif trait_args.nil?
          "#{trait_name.text}"
        else
          "#{trait_name.text}#{trait_args.text}"
        end

        params = params.text unless params.nil?
        return_type = return_type.text unless return_type.nil?


        location = Location.new(
          result.path,
          match["function.name"].range.start_point.row,
          match["function.name"].range.start_point.column
        )

        api = API.new(
          self,
          location,
          name.text,
          params,
          return_type
        )

        key = [trait, name.text]

        @api[key] = api
      end
    end

    # suppress output
    nil
  end

  class << self
    def register!(name, obj)
      @registry ||= {}
      @registry[name] = obj
    end

    def [](name)
      @registry[name]
    end

    def struct(name, location)
      new(name, location, :struct).tap { |o| register!(name, o) }
    end

    def enum(name, location)
      new(name, location, :enum).tap { |o| register!(name, o) }
    end

    def trait(name, location)
      new(name, location, :trait).tap { |o| register!(name, o) }
    end
  end

end


class API
  attr_accessor :parent, :location, :trait

  def initialize(parent, location, name, params, return_type, trait = nil)
    @parent = parent; @location = location; @name = name; @params = params; @return_type = return_type; @trait = trait
  end

  def returns?
    !@return_type.nil?
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
        require File.join('.', path)
        klass_name = Query.key_for(path)
        klass = Object.const_get(klass_name)
        @registry[klass_name] = klass.new(path)
      when path.end_with?('.scm.erb')
        # TODO: `run!` should take a context hash.
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
  end

  def run!
    SourceTree.query(self.code)
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
  def initialize(path)
    super
    @code = File.read(path)
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
        # TODO: Move #query to Entry
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

  # TODO: Entry should have #query, and also a creation method that takes raw source and parses it for querying. This
  # way I can execute queries on the results of other queries.
  class Entry
    attr_accessor :path

    def initialize(path)
      @path = path
    end

    def content
      @content ||= SourceTree.parse(File.read(@path))
    end
  end

end



SourceTree.load!
Query.load!

# results = Query[:Structs].run!
# results.each do |result|
#   # TODO: Move this into the query .scm somehow?
#   key = "struct.name"
#   result.matches.each do |match|
#     obj = match[key]
#     puts "#{result.path}:#{obj.range.start_point.row}: #{key.gsub(".name","")} #{obj.text}"
#   end
# end

puts ""
puts "=================="
puts ""


# TODO: impl.scm -> impl.scm.erb, and pass the type name through.
Query[:Impl].run!("Move").each do |result|
  result.matches.each do |match|
    puts "#{match["function.name"].text}#{match["function.parameters"].text}"
  end
end

Query[:Structs].run!.each do |result|
  match = result.matches[0]
  name = match["struct.name"]
  location = Location.new(
    result.path,
    name.range.start_point.row,
    name.range.start_point.column
  )
  Type.struct(name.text, location)
end

Query[:Enums].run!.each do |result|
  match = result.matches[0]
  name = match["enum.name"]
  location = Location.new(
    result.path,
    name.range.start_point.row,
    name.range.start_point.column
  )
  Type.enum(name.text, location)
end



Type["UCI"].find_apis!

binding.pry

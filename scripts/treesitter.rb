#TODO: Break this into files... I should probably make a gem. :/
#TODO: Issue when parsing types w/ lots of generics (like Familiar), check on that.
require 'async'
require 'async/barrier'
require 'find'
require 'pry'
require 'tree_sitter'
require 'tree_stand'


class Location
  attr_accessor :file, :line, :column

  def initialize(file, line, column)
    @file = file; @line = line; @column = column
  end
end

class Type
  # TODO: Look at this mess, clean up.
  attr_accessor :name, :location, :kind, :fields
  attr_reader :api

  # TODO: Fields. Removing the default should break the right stuff, extend the query, maybe connect w/ the
  # struct/enum/trait refactor?
  def initialize(name, location, kind, fields = {})
    @name = name; @location = location; @kind = kind ; @fields = fields; 
  end

  def find_apis!(refresh: false)
    @api = nil if refresh
    return @api unless @api.nil?

    @api = {}

    Query[:Impl].run!(self.name).each do |result|
      result.matches.each do |match|
        name = match["function.name"]
        next if name.nil?
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
          return_type,
          trait
        )

        key = [trait, name.text]

        @api[key] = api
      end
    end

    # suppress output
    nil
  end

  def uml_kind
    case self.kind
    when :struct
      "class"
    when :enum
      "enum"
    when :trait
      "interface"
    else
      raise "Unknown kind: #{self.kind}"
    end
  end

  def to_uml
    <<~DIA
      #{self.uml_kind} #{self.name} {
        .. fields ..
        #{self.fields.map(&:to_uml).join("\n  ")}
        .. methods ..
        #{self.api.map { |k, v| v.to_uml }.join("\n  ")} 
      }
    DIA
  end

  class << self
    def register!(name, obj)
      @registry ||= {}
      @registry[name] = obj
    end

    def types
      @registry.values
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
  attr_reader :parent, :location, :trait, :name, :params, :return_type

  def initialize(parent, location, name, params, return_type, trait = nil)
    @parent = parent; @location = location; @name = name; @params = params; @return_type = return_type; @trait = trait
  end

  def returns?
    !@return_type.nil?
  end

  def implements_trait?
    !@trait.nil?
  end

  def to_uml
    ret = "#{@name}#{@params}"
    ret = "#{@trait}::#{ret}" if self.implements_trait?
    ret = "#{ret} -> #{@return_type}" if self.returns?
    ret
  end
end

class Query
  attr_reader :src_path, :code

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

  def key
    self.class.key_for(@src)
  end

  class << self
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
    # 
    # TODO: Strategy pattern
    def load!
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

    def [](key)
      @registry[key]
    end

    def key_for(path)
      snake_case = File.basename(path).gsub('.scm', '').gsub('.rb', '')
      # "camel_case" -> :CamelCase
      snake_case.split('_').map(&:capitalize).join.to_sym
    end
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

# TODO: Marshall this and only load if the SHA has changed. -- Make a Marshall class, probably I should name this
# something, damn it I'm getting attached.
SourceTree.load!
Query.load!


barrier = Async::Barrier.new
Async do

  # TODO: These could be unified into a single query that returns any of them and extracts the :kind that way.
  barrier.async do
    STDERR.puts "Parsing Structs..."
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
  end

  barrier.async do
    STDERR.puts "Parsing Enums..."
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
  end

  barrier.async do
    STDERR.puts "Parsing Traits..."
    Query[:Traits].run!.each do |result|
      match = result.matches[0]
      name = match["trait.name"]
      location = Location.new(
        result.path,
        name.range.start_point.row,
        name.range.start_point.column
      )
      Type.trait(name.text, location)
    end
  end

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
  puts "@enduml"

  barrier.wait
end

STDERR.puts "Done!"



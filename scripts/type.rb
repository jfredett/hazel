class Type
  # TODO: Look at this mess, clean up.
  attr_accessor :name, :location, :kind, :fields
  attr_reader :api

  FILTERED_FIELD_TYPES = %w(u8 u16 u32 u64 i8 i16 i32 i64 isize usize bool str char String () Self)
  

  # FIXME: 15-JAN-2025 - Afternoon - Fields is currently really 'fields' or 'variants with optional fields', the latter
  # only if `kind` is :enum. These should be separate classes, but I end up in naming confliction with the query stuff.
  # Solution is to namespace query objects underneath `Query` and type stuff underneath `Type`, but that probably needs
  # a bigger refactor to separate files and maybe a separate project.
  #
  # 2218 - Put the queries transparently under `Query`. The way to refactor this is going to move to a module `Model`
  # under which I'll have `Struct, Field, Enum, Variant, Trait`, etc. Last will be to extract the database creation
  # stuff to it's own class that can then be responsible for creating and saving the results of queries.
  #
  # I also need to shove this in a better directory structure.
  #
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
      "struct"
    when :enum
      "enum"
    when :trait
      "interface"
    else
      raise "Unknown kind: #{self.kind}"
    end
  end

  def uml_links
    self.fields.values.map do |f|
      "\"#{f.type}\" *-- \"#{self.name}\"" unless self.skip_field?(f.type)
    end.reject(&:nil?).join("\n")
  end

  def skip_field?(field_type)
    FILTERED_FIELD_TYPES.include?(field_type)
  end

  def to_uml
    <<~DIA
      #{self.uml_kind} #{self.name} {
        .. fields ..
        #{self.fields.values.map(&:to_uml).join("\n  ")}
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
      return nil if @registry.nil?
      @registry["Query::#{name}".to_sym]
    end

    def struct(name, location, fields)
      new(name, location, :struct, fields).tap { |o| register!(name, o) }
    end

    def enum(name, location)
      new(name, location, :enum).tap { |o| register!(name, o) }
    end

    def trait(name, location)
      new(name, location, :trait).tap { |o| register!(name, o) }
    end
  end
end

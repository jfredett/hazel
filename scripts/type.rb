class Type
  # TODO: Look at this mess, clean up.
  attr_accessor :name, :location, :kind, :fields
  attr_reader :api

  # FIXME: Fields is currently really 'fields' or 'variants with optional fields', the latter only if `kind` is :enum. 
  # These should be separate classes, but I end up in naming confliction with the query stuff. Solution is to namespace
  # query objects underneath `Query` and type stuff underneath `Type`, but that probably needs a bigger refactor to
  # separate files and maybe a separate project.
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

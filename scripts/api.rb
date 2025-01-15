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

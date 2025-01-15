

class Field
  attr_reader :vis, :name, :type

  # TODO: Promote Type to an object reference back into the Type pool.
  def initialize(vis, name, type)
    @vis = vis; @name = name; @type = type
  end

  def to_uml
    "#{@vis}#{" " unless @vis.nil?}#{@name}: #{@type}"
  end
end


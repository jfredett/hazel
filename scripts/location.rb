
class Location
  attr_accessor :file, :line, :column

  def initialize(file, line, column)
    @file = file; @line = line; @column = column
  end
end


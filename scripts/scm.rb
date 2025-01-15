class SCM < Query
  def initialize(path)
    super
    @code = File.read(path)
  end
end

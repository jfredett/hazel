class Impl2 < SCM
  def code(type_name)
    <<~QUERY.strip
      (impl_item
        type: (type_identifier) @type.name
        body: (declaration_list) @impl.body
        (#eq? @type.name "#{type_name}"))
    QUERY
  end

  def run!(type_name)
    SourceTree.query(self.code(type_name))
  end
end

class Impl < SCM
  def code(type_name)
    <<~QUERY.strip
      (impl_item
        trait: (generic_type
          type: (type_identifier) @trait.name
          type_arguments: (type_arguments) @trait.args
        )?
        type: (type_identifier) @type.name
        body: (declaration_list
          (function_item
            name: (identifier) @function.name
            (parameters) @function.parameters)
            return_type: (type_identifier)? @function.return_type)
        (#eq? @type.name "#{type_name}"))
    QUERY
  end

  def run!(type_name)
    SourceTree.query(self.code(type_name))
  end
end

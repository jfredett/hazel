# Looks for Enums and adds them to the database
class Query
  class Enum < SCM
    def code
      <<~QUERY.strip
      [
        (enum_item
          (visibility_modifier)? @vis
          name: (type_identifier) @type
          body: (enum_variant_list
            [
              (enum_variant
                name: (identifier) @variant.name
                body: [
                  (field_declaration_list
                    (field_declaration
                      name: (field_identifier) @variant.field_name
                      type: (_) @variant.field_type
                      )*
                    )
                  (ordered_field_declaration_list
                    type: (_) @variant.field_type)
                ]
              )*
              (enum_variant
                name: (identifier) @variant.name
                value: (_) @variant.value
              )*
            ]
          )
        )
      ]
      QUERY
    end

    def run!
      super.each do |result|
        match = result.matches[0]
        name = match["type"]
        location = Location.new(
          result.path,
          name.range.start_point.row,
          name.range.start_point.column
        )
        # TODO: Implement variants, rough version below
        #
        variants = []
        # variants = match["variant.name"].map do |variant|
        #   fields = variant["field"].map do |field|
        #     vis = field["field.vis"].text if field.has_key?("field.vis")
        #     name = field["field_name"].text if field.has_key?("field_name")
        #     type = field["field_type"].text if field.has_key?("field_type")

        #     Field.new(vis, name, type)
        #   end
        #   Variant.new(variant["name"].text, fields)
        # end
        Type.send(:enum, name.text, location, variants)
      end
    end
  end
end

require 'tree_sitter'
require 'pry'


# parser = TreeSitter::Parser.new
# language = TreeSitter::Language.load('rust', '/nix/store/rp6435n8gklillifhyvdsq7z5sqzv8b2-tree-sitter-rust-grammar-0.24.4/parser')
# parser.language = language

# source_code = File.read('src/lib.rs')

# tree = parser.parse_string(nil, source_code)

require 'tree_stand'
require 'find'

TreeStand.configure do
  config.parser_path = '.parsers'
end

parser = TreeStand::Parser.new('rust')

# Load queries to a hash
queries = {
  structs_enums_traits: File.read('queries/structs_enums_traits.scm'),
}

source_files = Find.find('src') do |path|
  if File.directory?(path)
    Find.prune if File.basename(path).start_with?('.')
  else
    next unless path.end_with?('.rs')
    source_code = File.read(path)
    tree = parser.parse_string(source_code)
    matches = tree.query(queries[:structs_enums_traits])
    matches.each do |match|
      key = if match.has_key?("struct.name")
        "struct.name"
      elsif match.has_key?("enum.name")
        "enum.name"
      elsif match.has_key?("trait.name")
        "trait.name"
      end

      obj = match[key]

      puts "#{path}:#{obj.range.start_point.row}: #{key.gsub(".name","")} #{obj.text}"
    end
  end
end



# matches = tree.query(<<~QUERY)
#   (struct_item name: (type_identifier) @struct.name)
#   (enum_item name: (type_identifier) @enum.name)
#   (trait_item name: (type_identifier) @enum.name)
# QUERY


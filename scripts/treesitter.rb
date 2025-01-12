require 'tree_sitter'
require 'pry'


# parser = TreeSitter::Parser.new
# language = TreeSitter::Language.load('rust', '/nix/store/rp6435n8gklillifhyvdsq7z5sqzv8b2-tree-sitter-rust-grammar-0.24.4/parser')
# parser.language = language

# source_code = File.read('src/lib.rs')

# tree = parser.parse_string(nil, source_code)

require 'tree_stand'

TreeStand.configure do
  config.parser_path = '.parsers'
end

parser = TreeStand::Parser.new('rust')

source_code = File.read('src/coup/rep/mod.rs')

tree = parser.parse_string(source_code)

# Load queries to a hash
# queries = {
#   structs_enums_traits: File.read('queries/structs_enums_traits.scm'),
# }

matches = tree.query(<<~QUERY)
  (struct_item name: (type_identifier) @struct.name)
QUERY

matches.each do |match|
  struct_name = match["struct.name"].text
  puts struct_name
end

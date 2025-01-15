class SourceTree
  class << self
    attr_reader :sources

    def load!
      @sources = {}

      Find.find('src') do |path|
        Find.prune if File.directory?(path) && File.basename(path).start_with?('.')
        next unless path.end_with? '.rs'
        @sources[path] = Entry.new(path)
      end
    end

    def query(code)
      self.sources.map do |path, entry|
        # TODO: Move #query to Entry
        matches = entry.content.query(code)
        if matches.any?
          Result.new(path, entry.content.query(code))
        else
          nil
        end
      end.reject(&:nil?).flatten
    end

    def parse(code)
      self.parser.parse_string(code)
    end

    def parser
      return @parser if @parser

      ::TreeStand.configure do
        config.parser_path = '.parsers'
      end

      @parser = TreeStand::Parser.new('rust')
    end
  end

  class Result
    attr_accessor :path, :matches

    def initialize(path, matches)
      @path = path
      @matches = matches 
    end
  end

  # TODO: Entry should have #query, and also a creation method that takes raw source and parses it for querying. This
  # way I can execute queries on the results of other queries.
  class Entry
    attr_accessor :path

    def initialize(path)
      @path = path
    end

    def content
      @content ||= SourceTree.parse(File.read(@path))
    end
  end

end


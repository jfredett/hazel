class Query
  attr_reader :src_path, :code

  # TODO: this should support a filter over the find results. When executing I want to cache the parse if I can, but I
  # could generate a list of paths using another find, this assumes no changes during the generation, but locking is a
  # topic for another day (I'd probably go for an `inotify` or w/e the tool is).
  #
  # The filter would have to run at query time, ideally it would be separate from the query definition, maybe a separate
  # registry? `just treesitter filter_name query_name`?
  #
  # I can extract it from `just` and maybe make the filter dynamic.

  def initialize(path)
    @src = path
  end

  def run!
    SourceTree.query(self.code)
  end

  def key
    self.class.key_for(@src)
  end

  class << self
    # Find all the queries in the queries directory, and load them into a registry by name. Queries can be either `.scm`
    # or .`.rb` files. `.scm` files are assumed to be treesitter queries to be executed on the source tree and return
    # their matches from the #run! method.
    #
    # `.rb` files are assumed to be ducks of the Query class (you can subclass it to get most of the behavior for most
    # queries). This will let you implement your own `#run!` method, and you can refer to other queries by their name in
    # the registry a la `Query[:NameOfQuery].run!`.
    #
    # This method loads all these queries into a flat registry, nested directories are allowed, but the namespace is flat,
    # So you will need to ensure that your queries have unique names across the whole `queries` directory and it's
    # children.
    #
    # This loads all the queries underneath the `Query` constant to avoid name collisions.
    #
    # TODO: Drop support for .scm, just do .rb, and collapse SCM -> Query
    def load!
      @registry ||= {}
      Find.find('queries') do |path|
        Find.prune if File.directory?(path) && File.basename(path).start_with?('.')
        case
        when path.end_with?('.scm')
          scm = SCM.new(path)
          @registry[scm.key] = scm
        when path.end_with?('.rb')
          require File.join('.', path)
          klass_name = Query.key_for(path)
          klass = Query.const_get(klass_name)
          @registry[klass_name] = klass.new(path)
        else
          next
        end
      end
    end

    def [](key)
      @registry[key]
    end

    def key_for(path)
      snake_case = File.basename(path).gsub('.scm', '').gsub('.rb', '')
      # "camel_case" -> :CamelCase
      snake_case.split('_').map(&:capitalize).join.to_sym
    end
  end
end


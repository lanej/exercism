# frozen_string_literal: true

class Grep
  Match = Struct.new(:text, :line, :file, keyword_init: true)

  def self.grep(pattern, flags = [], files = [])
    line_numbers     = flags.include?('-n')
    ignore_case      = flags.include?('-i')
    match_line       = flags.include?('-x')
    invert_match     = flags.include?('-v')

    print_only_filenames = flags.include?('-l')
    print_filenames      = files.size > 1

    formatter = if print_only_filenames
                  lambda(&:file)
                elsif print_filenames && line_numbers
                  ->(match) { "#{match.file}:#{match.line + 1}:#{match.text}" }
                elsif line_numbers
                  ->(match) { "#{match.line + 1}:#{match.text}" }
                elsif print_filenames
                  ->(match) { "#{match.file}:#{match.text}" }
                else
                  lambda(&:text)
                end

    new(pattern, files,
        ignore_case: ignore_case,
        entire_line: match_line,
        invert: invert_match)
      .matches
      .map(&formatter)
      .join("\n")
  end

  attr_reader :matcher
  attr_reader :files

  def initialize(pattern, files, ignore_case: false, entire_line: false, invert: false)
    pattern = "^#{pattern}$" if entire_line
    pattern = Regexp.new(pattern, (ignore_case && Regexp::IGNORECASE))
    @files = files
    @matcher = invert ? ->(l) { !pattern.match?(l) } : ->(l) { pattern.match?(l) }
  end

  def matches
    files.each_with_object([]) do |file, matches|
      File.readlines(file).each_with_index do |line, i|
        line = line.strip
        next unless matcher.call(line)

        matches << Match.new(text: line, line: i, file: file)
      end
    end
  end
end

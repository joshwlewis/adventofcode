module Bingo
  def self.build_game(input)
    numbers = []
    map = []
    locations = {}
    row = 0
    input.each_line.with_index do |line, i|
      if i == 0
        numbers = line.split(',').map(&:to_i)
        next
      end

      next unless line
      line = line.strip
      next if line.empty?

      nums = line.split.map(&:to_i)
      map.push(nums)
      nums.each_with_index do |num, col|
        loc = Location.new(map.count - 1, col)
        if locations[num]
          locations[num].push(loc)
        else
          locations[num] = [loc]
        end
      end
    end
    return Game.new(numbers, map, locations)
  end

  class Game
    def initialize(numbers, map, locations)
      @numbers = numbers
      @map = map
      @locations = locations
      @row_marks = Array.new(map.count, 0)
      @col_marks = Array.new(map.count, 0)
    end

    def play
      (0..numbers.count).each do |turn|
        play_turn(turn)
        if winner = check_winner
          return winner, numbers[turn]
        end
      end
    end

    def score(board)
      rowstart = board*5
      sum = 0
      (rowstart..rowstart+4).reduce(0) do |acc, r|
        acc + map[r].compact.sum
      end
    end

    private

    attr_reader :numbers, :locations
    attr_accessor :map, :row_marks, :col_marks

    def play_turn(i)
      num = numbers[i]
      locations[num].each do |loc|
        map[loc.row][loc.col] = nil
        row_marks[loc.row] += 1
        col_marks[loc.col+((loc.row/5)*5)] += 1
      end
    end

    def check_winner
      [row_marks, col_marks].each do |marks|
        win_i = marks.find_index{|mc| mc >= 5}
        if win_i
          return win_i / 5
        end
      end
      return nil
    end
  end

  class Location
    attr_reader :row, :col
    def initialize(row, col)
      @row = row
      @col = col
    end

    def inspect
      "<#{row}:#{col}>"
    end
  end
end


require 'set'

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
      @winners = Set.new()
    end

    def play_to_win
      numbers.each do |num|
        play_turn(num)
        winners.each do |winner|
          return winner, num
        end
      end
    end

    def play_to_lose
      losing_board = nil
      numbers.each do |num|
        play_turn(num)
        if winners.count == board_count - 1
          (0...board_count).each do |board_num|
            if !winners.include?(board_num)
              losing_board = board_num
            end
          end
        end
        if winners.count == board_count
          return losing_board, num
        end
      end
    end

    def score(board)
      rowstart = board*5
      sum = 0
      (rowstart...rowstart+5).reduce(0) do |acc, r|
        acc + map[r].compact.sum
      end
    end

    private

    attr_reader :numbers, :locations
    attr_accessor :map, :row_marks, :col_marks, :winners

    def board_count
      map.count / 5
    end

    def play_turn(num)
      locations[num].each do |loc|
        board = loc.row/5
        next if winners.include?(board)
        map[loc.row][loc.col] = nil
        row_marks[loc.row] += 1
        col_num = (board*5) + loc.col
        col_marks[col_num] += 1
        if row_marks[loc.row] >= 5 || col_marks[col_num] >= 5
          winners.add(board)
        end
      end
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


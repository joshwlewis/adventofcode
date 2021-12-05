require 'minitest/autorun'
require_relative './lib.rb'

SAMPLE_DATA = <<~DATA
  7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

  22 13 17 11  0
    8  2 23  4 24
  21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
  19  8  7 25 23
  20 11 10 24  4
  14 21 16 12  6

  14 21 17 24  4
  10 16 15  9 19
  18  8 23 26 20
  22 11 13  6  5
    2  0 12  3  7
DATA

class BingoTest < Minitest::Test
  def test_bingo_win
    game = Bingo.build_game(SAMPLE_DATA)
    winner, number = game.play_to_win
    assert_equal 2, winner
    assert_equal 24, number

    score = game.score(winner)
    assert_equal 188, score

  end
  def test_bingo_lose
    game = Bingo.build_game(SAMPLE_DATA)
    loser, number = game.play_to_lose
    assert_equal 1, loser
    assert_equal 13, number

    score = game.score(loser)
    assert_equal 148, score
  end
end


require_relative './lib.rb'

input = File.read('./input.txt')

game = Bingo.build_game(input)
winner, winning_number = game.play_to_win
winner_score = game.score(winner)
puts "winner: board_number: #{winner}, number: #{winning_number}, score: #{winner_score}, sum: #{winner_score*winning_number}"

game = Bingo.build_game(input)
loser, losing_number = game.play_to_lose
loser_score = game.score(loser)
puts "loser: board_number: #{loser}, number: #{losing_number}, score: #{loser_score}, sum: #{loser_score*losing_number}"


require_relative './lib.rb'

input = File.read('./input.txt')

game = Bingo.build_game(input)
winner, number = game.play
score = game.score(winner)
puts "winner: #{winner}, number: #{number}, score: #{score}, sum: #{score*number}"

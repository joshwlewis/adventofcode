last_depth = nil
basic_increases = 0
File.readlines('./input.txt').each do |line|
  depth = line.strip.to_i
  basic_increases+=1 if last_depth && depth > last_depth
  last_depth = depth
end
puts "Part 1: #{basic_increases}"

window = []
window_increases = 0
last_sum = nil
File.readlines('./input.txt').each do |line|
  depth = line.strip.to_i
  window.push(depth)
  if window.count > 3
    window.shift
  end
  if window.count == 3
    sum = window.sum
    window_increases+=1 if last_sum && sum > last_sum
    last_sum = sum
  end
end

puts "Part 2: #{window_increases}"

with_fx :reverb, mix: 0.2 do
  loop do
    play scale(:Eb2, :major_pentatonic, num_octaves: 3).choose,
      release: 0.1,
      amp: rand
    sleep 0.1
  end
end

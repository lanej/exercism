local Hamming = {}

function Hamming.compute(a,b)
  if string.len(a) ~= string.len(b) then
    return -1
  end

  local hamming = 0

  for i = 1, #a do
    if a:sub(i,i) ~= b:sub(i,i) then
      hamming = hamming + 1
    end
  end

  return hamming
end

return Hamming

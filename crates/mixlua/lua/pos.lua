Position = {}
Position.__index = Position

function Position.new(x, y)
    return setmetatable({ x = x, y = y }, Position)
end

function Position:add(other)
    return Position.new(self.x + other.x, self.y + other.y)
end

-- Overload `__tostring` meta method
function Position:__tostring()
    return "Position(" .. self.x .. ", " .. self.y .. ")"
end

local pos_a = Position.new(1, 2)
local pos_b = Position.new(5, 3)

local pos_add = pos_a:add(pos_b)
print(pos_add)

-- Test: convert binary(string) to number
local bin = "\x12\xef"
print("bin.len: " .. string.len(bin))
local num = string.byte(bin, 2) | (string.byte(bin, 1) << 8)
print(string.format("%#x", num))

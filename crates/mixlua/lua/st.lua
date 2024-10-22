require("init")

Structure = {}
Structure.__index = Structure

function Structure.new_inner(bytes)
    local _temp = {
        { name = "index", size = 4 },
        { name = "byte", size = 1 },
        { name = "uns", size = 2 },
    }

    return setmetatable({ inner = SetupTableData(bytes, true, true, _temp) }, Structure)
end

-- Overload `__tostring` meta method
function Structure:__tostring()
    return PrintTable(self.inner)
end

-- print(Structure.new_inner("\x04\x00\x00\x00\x0b\x03\xa1\x0a"))

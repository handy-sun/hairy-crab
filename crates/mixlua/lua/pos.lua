
-- Convert binary(string) to number
function Bytes2int(bytes, is_little_endian)
    local fmt = (is_little_endian == true and "<I" or ">I") .. #bytes
    return string.unpack(fmt, bytes)
end

function SetupTableData(bytes, tab_list)
    local index = 1
    local new_list = {}
    for _, value in pairs(tab_list) do
        if index > #bytes then
            break
        end
        local part_size = value.size
        local next_index = index + part_size
        local part_bytes = string.sub(bytes, index, next_index - 1)

        table.insert(new_list, {
            name = value.name,
            size = value.size,
            data = Bytes2int(part_bytes, true)
        })

        index = next_index
    end
    return new_list
end

function PrintTable(t, indent)
    indent = indent or 0
    local prefix = string.rep("  ", indent)
    local str = ""
    for key, value in pairs(t) do
        if type(value) == "table" then
            str = str .. (prefix .. tostring(key) .. ':\n')
            str = str .. PrintTable(value, indent + 1)
        else
            str = str .. (prefix .. tostring(key) .. ": " .. tostring(value) .. '\n')
        end
    end
    return str
end

Structure = {}
Structure.__index = Structure

function Structure.new(bytes)
    local _temp = {
        { name = "index", size = 4 },
        { name = "byte", size = 1 },
        { name = "uns", size = 2 },
    }

    return setmetatable({ inner = SetupTableData(bytes, _temp) }, Structure)
end

-- Overload `__tostring` meta method
function Structure:__tostring()
    return PrintTable(self.inner)
end

-- local bytes = "\x04\x00\x01\x00\x0b\x03\x01\x0a"
-- local test_stru = Structure.new(bytes)

-- print(test_stru)

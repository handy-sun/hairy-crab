
-- Convert binary(string) to number
function Bytes2int(bytes, is_little_endian)
    local fmt = (is_little_endian == true and "<I" or ">I") .. #bytes
    return string.unpack(fmt, bytes)
end

function SetupTableData(bytes, is_little_endian, is_array, tab_list)
    local index = 1
    local new_list = {}
    while true do
        for _, value in pairs(tab_list) do
            if index > #bytes then
                return new_list
            end
            local part_size = value.size
            local next_index = index + part_size
            local part_bytes = string.sub(bytes, index, next_index - 1)

            table.insert(new_list, {
                name = value.name,
                size = value.size,
                data = Bytes2int(part_bytes, is_little_endian)
            })

            index = next_index
        end
    end
end

function PrintTable(t, indent)
    indent = indent or 0
    local prefix = string.rep("  ", indent)
    local str_list = {}
    for key, value in pairs(t) do
        if type(value) == "table" then
            table.insert(str_list, string.format("%s%s:\n", prefix, tostring(key)))
            table.insert(str_list, PrintTable(value, indent + 1))
        else
            table.insert(str_list, string.format("%s%s: %s\n", prefix, tostring(key), tostring(value)))
        end
    end
    return table.concat(str_list, '')
end

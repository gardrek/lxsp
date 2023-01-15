local dbg = {}

function dbg.to_debug_string(value)
    if type(value) == "string" then
        return dbg.to_source_string(value)
    else
        return tostring(value)
    end
end

function dbg.to_source_string(value)
    return string.gsub(value, "([\"'])", "\\%1")
end

function dbg.sub_escapes(string)
    return tostring(value)
end

return dbg

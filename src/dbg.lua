local dbg = {}

local mt = {}

local global = dofile"src/gmt.lua"

function dbg.to_debug_string(value)
    if false then
        error""
    elseif type(value) == "nil" then
        return "[Lua nil]"
    elseif type(value) == "boolean" then
        return "[Lua bool:" .. tostring(value) .. "]"
    elseif type(value) == "number" then
        return "[Lua number:" .. tostring(value) .. "]"
    elseif type(value) == "string" then
        return "[String:" .. dbg.to_source_string(value) .. "]"
    elseif type(value) == "table" then
        return "[table:" .. tostring(value) .. "]"
    elseif type(value) == "function" then
        return "[Lua Function:" .. tostring(value) .. "]"
    elseif type(value) == "CFunction" then
        return "[Lua FFI Function:" .. tostring(value) .. "]"
    elseif type(value) == "userdata" then
        return "[Lua FFI Userdata:" .. tostring(value) .. "]"
    else
        error("unhandled primitive Lua type [=[ " .. type(value) .. " ]=]")
    end
end

function dbg.to_source_string(value)
    return '"' .. string.gsub(value, "([\"'])", "\\%1") .. '"'
end

function dbg.sub_escapes(string)
    return tostring(value)
end

return dbg
